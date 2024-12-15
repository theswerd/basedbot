import os
import math
import json
import time
import traceback
import requests
from dotenv import load_dotenv
import cv2
import mediapipe as mp
import numpy as np
from PIL import Image
from mediapipe.framework.formats import landmark_pb2
from depth import DepthModel
from types import MappingProxyType

import rerun as rr
import rerun.blueprint as rrb

load_dotenv()
STREAM_ENDPOINT = os.getenv('STREAM_ENDPOINT', 'http://localhost:8080')


if os.path.exists('/dev/video0'):
    _camera = '/dev/video0'
else:
    _camera = 0

MEDIAPIPE_POSE_LANDMARKS = MappingProxyType({
    # Face/Head
    0: "nose",
    1: "left_eye_inner",
    2: "left_eye",
    3: "left_eye_outer",
    4: "right_eye_inner",
    5: "right_eye",
    6: "right_eye_outer",
    7: "left_ear",
    8: "right_ear",
    9: "mouth_left",
    10: "mouth_right",

    # Arms
    11: "left_shoulder",
    12: "right_shoulder",
    13: "left_elbow",
    14: "right_elbow",
    15: "left_wrist",
    16: "right_wrist",
    17: "left_pinky",
    18: "right_pinky",
    19: "left_index",
    20: "right_index",
    21: "left_thumb",
    22: "right_thumb",

    # Torso/Legs
    23: "left_hip",
    24: "right_hip",
    25: "left_knee",
    26: "right_knee",
    27: "left_ankle",
    28: "right_ankle",
    29: "left_heel",
    30: "right_heel",
    31: "left_foot_index",
    32: "right_foot_index"
})

POSE_CONNECTIONS = tuple([
    (11, 12),  # Left shoulder to right shoulder
    (11, 13),  # Left shoulder to left elbow
    (13, 15),  # Left elbow to left wrist
    (12, 14),  # Right shoulder to right elbow
    (14, 16),  # Right elbow to right wrist
    (11, 23),  # Left shoulder to left hip
    (12, 24),  # Right shoulder to right hip
    (23, 24),  # Left hip to right hip
    (23, 25),  # Left hip to left knee
    (25, 27),  # Left knee to left ankle
    (24, 26),  # Right hip to right knee
    (26, 28),  # Right knee to right ankle
])

BOT_POSE_LANDMARKS = MappingProxyType({
    15: "left_shoulder_yaw",
    12: "right_shoulder_yaw",
    11: "right_elbow_yaw",
    16: "left_elbow_yaw",
    13: "right_shoulder_pitch",
    14: "left_shoulder_pitch",
})

_parts_to_plot = BOT_POSE_LANDMARKS.keys()
PLOT_PATHS = {
    idx: f"/series/keypoints/{value}" 
    for idx, value in BOT_POSE_LANDMARKS.items() if idx in _parts_to_plot
}

def log_pose_to_rerun(pose):
    for idx, landmark in enumerate(pose):
        # Create position for the bounding box center
        label = MEDIAPIPE_POSE_LANDMARKS.get(idx, f"Unknown_{idx}")

        if (landmark.visibility < 0.5):
            rr.log(f"/keypoints/pose_point_{label}", rr.Clear(recursive=True))
            continue

        # Log the corresponding point for reference
        positions = np.array([[landmark.x, landmark.y, landmark.z]])
        # yellow color
        colors = np.array([[255, 255, 0]])
        rr.log(f"/keypoints/pose_point_{label}", rr.Points3D(
            positions, colors=colors, radii=0.01, labels=[label]))


def log_landmark_data_to_rerun(landmark_data):
    for key, value in landmark_data.items():
        key = int(key)
        if key in _parts_to_plot:
            rr.set_time_nanos("log_time", int(time.time() * 1e9))
            plot_path = PLOT_PATHS[key]
            rr.log(plot_path, rr.Scalar(value))


def log_image_data_to_rerun(frame, depth_map):
    rr.log("/image/tracked_keypoints", rr.Image(frame, color_model="bgr"))
    rr.log("/image/depth_map", rr.Image(depth_map))


def right_yaw_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    denominator = max(abs(pose[12].y - pose[14].y), 1e-6)
    angle = math.degrees(math.atan(
        abs(pose[12].x - pose[14].x) / denominator))
    print(f"right shoulder side angle: {angle:.2f}")
    return angle


def left_yaw_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    denominator = max(abs(pose[11].y - pose[13].y), 1e-6)
    angle = math.degrees(math.atan(
        abs(pose[11].x - pose[13].x) / denominator))
    print(f"left shoulder side angle: {angle:.2f}")
    return angle


def right_pitch_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    denominator = max(abs(pose[12].y - pose[14].y), 1e-6)
    angle = math.degrees(math.atan(
        abs(pose[12].z - pose[16].z) / denominator))
    return angle


def left_pitch_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    denominator = max(abs(pose[11].y - pose[13].y), 1e-6)
    return math.degrees(math.atan(
        (pose[11].z - pose[15].z) / denominator))


def right_elbow_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    a = np.array([abs(pose[12].x - pose[14].x), abs(pose[12].y - pose[14].y)])
    b = np.array([abs(pose[16].x - pose[14].x), abs(pose[16].y - pose[14].y)])
    norm_product = max(np.linalg.norm(a) * np.linalg.norm(b), 1e-6)
    angle = math.degrees(math.acos(float(np.dot(a, b)) / norm_product))
    print(f"right elbow angle: {angle:.2f}")
    return angle


def left_elbow_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    a = np.array([abs(pose[11].x - pose[13].x), abs(pose[11].y - pose[13].y)])
    b = np.array([abs(pose[15].x - pose[13].x), abs(pose[15].y - pose[13].y)])
    norm_product = np.linalg.norm(a) * np.linalg.norm(b)
    norm_product = max(norm_product, 1e-6)
    angle = math.degrees(math.acos(float(np.dot(a, b)) / norm_product))
    print(f"left elbow angle: {angle:.2f}")
    return angle


def compute_angles(pose: list[landmark_pb2.NormalizedLandmark]):
    landmark_output = {
        '15': left_yaw_angle(pose),
        '12': right_yaw_angle(pose),
        '11': right_elbow_angle(pose),
        '16': left_elbow_angle(pose),
        '13': right_pitch_angle(pose),
        '14': left_pitch_angle(pose),
    }

    return landmark_output

def low_pass_filter(data, alpha=0.1):
    filtered_data = [data[0]]
    filtered_data.extend(
        {key: alpha * data[i][key] + (1 - alpha) * filtered_data[i - 1][key] for key in data[i]}
        for i in range(1, len(data))
    )
    return filtered_data


def print_landmark_positions(result):
    if result.pose_landmarks:
        pose = result.pose_landmarks[0]  # First person detected
        for idx, landmark in enumerate(pose):
            name = MEDIAPIPE_POSE_LANDMARKS.get(idx, f"Unknown_{idx}")
            print(
                f"{name}: x={landmark.x:.2f}, y={landmark.y:.2f}, z={landmark.z:.2f}")


def clip_pose_data(pose_data):
    clip_value = [0, 90]
    for key, value in pose_data.items():
        if value < clip_value[0]:
            pose_data[key] = clip_value[0]
        elif value > clip_value[1]:
            pose_data[key] = clip_value[1]
    return pose_data


def draw_landmarks_with_labels(rgb_image, detection_result):
    annotated_image = np.copy(rgb_image)

    if detection_result.pose_landmarks:
        pose = detection_result.pose_landmarks[0]  # First person detected

        # Draw connections first (so they appear behind the points)
        pose_landmarks_proto = landmark_pb2.NormalizedLandmarkList()
        pose_landmarks_proto.landmark.extend([
            landmark_pb2.NormalizedLandmark(
                x=landmark.x, y=landmark.y, z=landmark.z)
            for landmark in pose
        ])

        mp.solutions.drawing_utils.draw_landmarks(
            annotated_image,
            pose_landmarks_proto,
            mp.solutions.pose.POSE_CONNECTIONS,
            mp.solutions.drawing_styles.get_default_pose_landmarks_style())

        # Draw points and labels
        for idx, landmark in enumerate(pose):
            # Convert normalized coordinates to pixel coordinates
            x = int(landmark.x * annotated_image.shape[1])
            y = int(landmark.y * annotated_image.shape[0])

            # Draw larger point
            cv2.circle(annotated_image, (x, y), 5, (0, 255, 0), -1)

            # Add index and name
            label = f"{idx}:{MEDIAPIPE_POSE_LANDMARKS[idx]}"

            # Make text background black for better visibility
            (w, h), _ = cv2.getTextSize(label, cv2.FONT_HERSHEY_SIMPLEX, 0.5, 1)
            cv2.rectangle(annotated_image, (x+5, y-h-5),
                          (x+w+5, y+5), (0, 0, 0), -1)

            # Draw white text
            cv2.putText(annotated_image, label, (x+5, y),
                        cv2.FONT_HERSHEY_SIMPLEX, 0.5, (255, 255, 255), 1)

    return annotated_image


def main(stream: str = True):
    print("Starting main")
    rr.init("rerun_example_my_data", spawn=True)
    rr.log("/", rr.ViewCoordinates.RIGHT_HAND_Y_DOWN, static=True)
    # Initialize MediaPipe components
    BaseOptions = mp.tasks.BaseOptions
    PoseLandmarker = mp.tasks.vision.PoseLandmarker
    PoseLandmarkerOptions = mp.tasks.vision.PoseLandmarkerOptions
    VisionRunningMode = mp.tasks.vision.RunningMode

    def modify_z_coordinates(pose, depth_map):
        for landmark in pose:
            # Convert normalized coordinates to pixel indices
            pixel_y = int(landmark.y * frame.shape[0])
            pixel_x = int(landmark.x * frame.shape[1])

            # Clip indices to ensure they are within the valid range of the depth map
            pixel_y = np.clip(pixel_y, 0, depth_map.shape[0] - 1)
            pixel_x = np.clip(pixel_x, 0, depth_map.shape[1] - 1)

            # Assign the depth value to the z-coordinate
            landmark.z = depth_map[pixel_y, pixel_x]

    options = PoseLandmarkerOptions(
        base_options=BaseOptions(
            model_asset_path="pose_landmarker_lite.task"),
        output_segmentation_masks=True,
        running_mode=VisionRunningMode.IMAGE
    )

    camera = cv2.VideoCapture(_camera)
    pose_data = []
    depth_model = DepthModel()
    try:
        with PoseLandmarker.create_from_options(options) as landmarker:
            while True:
                if not camera.isOpened():
                    continue
                ret, frame = camera.read()

                # downsample 4x on the image
                frame = cv2.resize(
                    frame, (frame.shape[1] // 4, frame.shape[0] // 4))

                if not ret:
                    print("Failed to get frame after all retries")
                    break

                rgb_frame = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
                mp_image = mp.Image(
                    image_format=mp.ImageFormat.SRGB, data=rgb_frame)

                # Get detection results directly
                detection_result = landmarker.detect(mp_image)
                pil_image = Image.fromarray(frame)
                depth_map = depth_model.pred_depth(pil_image)

                # Draw landmarks if available
                frame = draw_landmarks_with_labels(frame, detection_result)

                if detection_result.pose_world_landmarks:
                    pose = detection_result.pose_world_landmarks[0]
                    image_pose = detection_result.pose_landmarks[0]
                    modify_z_coordinates(image_pose, depth_map)
                    log_pose_to_rerun(pose)
                    landmark_data = compute_angles(pose)
                    log_landmark_data_to_rerun(landmark_data)
                    landmark_data = clip_pose_data(landmark_data)
                    landmark_data = low_pass_filter([landmark_data])[0]
                    if stream:
                        requests.post(STREAM_ENDPOINT,
                                      json=landmark_data,
                                      headers={
                                          'Content-Type': 'application/json'},
                                      timeout=0.1)
                    else:
                        pose_data.append(landmark_data)

                depth_map_rendered = (depth_map * 100).astype(np.uint8)
                log_image_data_to_rerun(frame, depth_map_rendered)

                if cv2.waitKey(1) & 0xFF == ord('q'):
                    break

    except KeyboardInterrupt:
        print("Interrupted by user")
    except Exception as e:
        print(f"An error occurred: {e}")
        traceback.print_exc()  # Print the full traceback

    finally:
        with open('pose_data.json', 'w') as f:
            json.dump(pose_data, f, indent=4)
        camera.release()
        cv2.destroyAllWindows()

if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('--stream', action='store_true')
    args = parser.parse_args()
    main(args.stream)
