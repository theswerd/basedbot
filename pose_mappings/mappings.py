import os
import json
import math
import cv2
import mediapipe as mp
import numpy as np
from PIL import Image
from mediapipe.framework.formats import landmark_pb2
from depth import DepthModel
import traceback


import rerun as rr

if os.path.exists('/dev/video0'):
    _camera = '/dev/video0'
else:
    _camera = 0

POSE_LANDMARKS = {
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
}


# def log_pose_to_rerun(pose):
#     for idx, landmark in enumerate(pose):
#         positions = np.array([[landmark.x, landmark.y, landmark.z]])
#         colors = np.zeros_like(positions, dtype=np.uint8)
#         # colors[:, 0] = np.linspace(0, 255, len(pose))  # Assign gradient colors
#         # breakpoint()
#         rr.log(f"pose_points_{POSE_LANDMARKS[idx]}", rr.Points3D(
#             positions, colors=colors, radii=0.02))

def log_pose_to_rerun(pose):
    for idx, landmark in enumerate(pose):
        # Create position for the bounding box center
        center = np.array([[landmark.x, landmark.y, landmark.z]])
        # Define a small half-size for the bounding box
        half_size = [0.01, 0.01, 0.01]
        # Assign a label to each bounding box
        label = POSE_LANDMARKS.get(idx, f"Unknown_{idx}")

        # Log the bounding box
        rr.log(f"pose_box_{label}", rr.Boxes3D(
            centers=center,
            half_sizes=[half_size],
            radii=0.01,
            colors=[(255, 255, 0)],  # Yellow color for bounding boxes
            labels=[label]
        ))

        # Log the corresponding point for reference
        positions = np.array([[landmark.x, landmark.y, landmark.z]])
        colors = np.zeros_like(positions, dtype=np.uint8)
        rr.log(f"pose_point_{label}", rr.Points3D(
            positions, colors=colors, radii=0.02))


def right_shoulder_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    denominator = abs(pose[12].y - pose[14].y)
    if denominator < 1e-6:
        denominator = 1e-6
    return math.degrees(math.atan(
        abs(pose[12].x - pose[14].x) / denominator))


def left_shoulder_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    denominator = abs(pose[11].y - pose[13].y)
    if denominator < 1e-6:
        denominator = 1e-6
    return math.degrees(math.atan(
        abs(pose[11].x - pose[13].x) / denominator))


def right_shoulder_forward_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    denominator = abs(pose[12].y - pose[14].y)
    if denominator < 1e-6:
        denominator = 1e-6
    angle = math.degrees(math.atan(
        (pose[12].z - pose[14].z) / denominator))
    print("y12 , 14", pose[12].y, pose[14].y)
    print("z12 , 14", pose[12].z, pose[14].z)
    print("angle", angle)
    return angle


def left_shoulder_forward_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    denominator = abs(pose[11].y - pose[13].y)
    if denominator < 1e-6:
        denominator = 1e-6
    return math.degrees(math.atan(
        abs(pose[11].z - pose[13].z) / denominator))


def right_elbow_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    a = np.array([abs(pose[12].x - pose[14].x), abs(pose[12].y - pose[14].y)])
    b = np.array([abs(pose[16].x - pose[14].x), abs(pose[16].y - pose[14].y)])
    norm_product = np.linalg.norm(a) * np.linalg.norm(b)
    if norm_product < 1e-6:
        norm_product = 1e-6
    return math.degrees(math.acos(float(np.dot(a, b)) / norm_product))


def left_elbow_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    a = np.array([abs(pose[12].x - pose[14].x), abs(pose[12].y - pose[14].y)])
    b = np.array([abs(pose[16].x - pose[14].x), abs(pose[16].y - pose[14].y)])
    norm_product = np.linalg.norm(a) * np.linalg.norm(b)
    if norm_product < 1e-6:
        norm_product = 1e-6
    return math.degrees(math.acos(float(np.dot(a, b)) / norm_product))


def compute_angles(pose: list[landmark_pb2.NormalizedLandmark]):
    landmark_output = {
        '15': right_shoulder_angle(pose),
        '12': left_shoulder_angle(pose),
        '11': right_elbow_angle(pose),
        '16': left_elbow_angle(pose),
        '13': right_shoulder_forward_angle(pose),
        '14': left_shoulder_forward_angle(pose),
    }

    return landmark_output


def print_landmark_positions(result):
    if result.pose_landmarks:
        pose = result.pose_landmarks[0]  # First person detected
        for idx, landmark in enumerate(pose):
            name = POSE_LANDMARKS.get(idx, f"Unknown_{idx}")
            print(
                f"{name}: x={landmark.x:.2f}, y={landmark.y:.2f}, z={landmark.z:.2f}")


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
            label = f"{idx}:{POSE_LANDMARKS[idx]}"

            # Make text background black for better visibility
            (w, h), _ = cv2.getTextSize(label, cv2.FONT_HERSHEY_SIMPLEX, 0.5, 1)
            cv2.rectangle(annotated_image, (x+5, y-h-5),
                          (x+w+5, y+5), (0, 0, 0), -1)

            # Draw white text
            cv2.putText(annotated_image, label, (x+5, y),
                        cv2.FONT_HERSHEY_SIMPLEX, 0.5, (255, 255, 255), 1)

    return annotated_image


def main():
    print("Starting main")
    rr.init("rerun_example_my_data", spawn=True)
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
                    pose_data.append(landmark_data)
                # breakpoint()

                # Display frame
                cv2.imshow("MediaPipe Pose Landmarker", frame)
                cv2.imshow("Depth Map", depth_map / 100.0)
                cv2.moveWindow("MediaPipe Pose Landmarker", 0, 0)
                cv2.moveWindow("Depth Map", frame.shape[1], 0)

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
    main()
