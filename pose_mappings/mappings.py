import math
import cv2
import mediapipe as mp
import numpy as np
import time
import atexit
from PIL import Image
from mediapipe.framework.formats import landmark_pb2
from depth import DepthModel

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


def right_shoulder_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    return math.degrees(math.atan(
        abs(pose[12].x - pose[14].x) / abs(pose[12].y - pose[14].y)))


def left_shoulder_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    return math.degrees(math.atan(
        abs(pose[11].x - pose[13].x) / abs(pose[11].y - pose[13].y)))


def right_shoulder_forward_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    return math.degrees(math.atan(
        abs(pose[12].z - pose[14].z) / abs(pose[12].y - pose[14].y)))


def left_shoulder_forward_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    return math.degrees(math.atan(
        abs(pose[11].z - pose[13].z) / abs(pose[11].y - pose[13].y)))


def right_elbow_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    a = np.array([abs(pose[12].x - pose[14].x), abs(pose[12].y - pose[14].y)])
    b = np.array([abs(pose[16].x - pose[14].x), abs(pose[16].y - pose[14].y)])
    return math.degrees(math.acos(float(np.dot(a, b)) / (np.linalg.norm(a) * np.linalg.norm(b))))


def left_elbow_angle(pose: list[landmark_pb2.NormalizedLandmark]):
    a = np.array([abs(pose[12].x - pose[14].x), abs(pose[12].y - pose[14].y)])
    b = np.array([abs(pose[16].x - pose[14].x), abs(pose[16].y - pose[14].y)])
    return math.degrees(math.acos(float(np.dot(a, b)) / (np.linalg.norm(a) * np.linalg.norm(b))))


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
    # Initialize MediaPipe components
    BaseOptions = mp.tasks.BaseOptions
    PoseLandmarker = mp.tasks.vision.PoseLandmarker
    PoseLandmarkerOptions = mp.tasks.vision.PoseLandmarkerOptions
    VisionRunningMode = mp.tasks.vision.RunningMode

    latest_result = None

    def result_callback(result, output_image: mp.Image, timestamp_ms: int):
        # result is a mediapipe image
        nonlocal latest_result
        latest_result = {
            "joints_2d": result,
            "depth_map": None
        }

    def modify_z_coordinates(pose, depth_map):
        for landmark in pose:
            pixel_y = int(landmark.y * frame.shape[0])
            pixel_x = int(landmark.x * frame.shape[1])
            landmark.z = depth_map[pixel_y][pixel_x]

    options = PoseLandmarkerOptions(
        base_options=BaseOptions(model_asset_path="pose_landmarker_lite.task"),
        output_segmentation_masks=True,
        running_mode=VisionRunningMode.IMAGE
    )

    camera = cv2.VideoCapture(0)
    timestamp = 0
    pose_data = []
    depth_model = DepthModel()
    try:
        with PoseLandmarker.create_from_options(options) as landmarker:
            while True:
                if not camera.isOpened():
                    continue
                ret, frame = camera.read()

                # downsample 4x on the image
                frame = cv2.resize(frame, (frame.shape[1] // 4, frame.shape[0] // 4))

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
                    modify_z_coordinates(pose, depth_map)
                    landmark_data = compute_angles(pose)
                    pose_data.append(landmark_data)

                # Display frame
                cv2.imshow("MediaPipe Pose Landmarker", frame)
                cv2.imshow("Depth Map", depth_map)
                cv2.moveWindow("MediaPipe Pose Landmarker", 0, 0) 
                cv2.moveWindow("Depth Map", frame.shape[1], 0) 

                if cv2.waitKey(1) & 0xFF == ord('q'):
                    break

    except KeyboardInterrupt:
        # Export pose data to JSON file
        import json
        with open('pose_data.json', 'w') as f:
            json.dump(pose_data, f, indent=4)

        print("Interrupted by user")
    except Exception as e:
        print(f"An error occurred: {e}")
    finally:
        cv2.destroyAllWindows()


if __name__ == "__main__":
    main()


# def draw_world_landmarks_on_image(rgb_image, detection_result):
#     pose_landmarks_list = detection_result.pose_world_landmarks
#     annotated_image = np.copy(rgb_image)

#     # Loop through the detected poses to visualize.
#     for idx in range(len(pose_landmarks_list)):
#         pose_landmarks = pose_landmarks_list[idx]

#         # Scale world coordinates to image coordinates
#         h, w = rgb_image.shape[:2]
#         scaled_landmarks = []
#         for landmark in pose_landmarks:
#             # Scale x and y to image dimensions, discard z
#             x = int(landmark.x * w)
#             y = int(landmark.y * h)
#             scaled_landmarks.append((x, y))

#         # Draw connections between landmarks
#         for connection in mp.solutions.pose.POSE_CONNECTIONS:
#             start_idx = connection[0]
#             end_idx = connection[1]
#             cv2.line(annotated_image,
#                      scaled_landmarks[start_idx],
#                      scaled_landmarks[end_idx],
#                      (0, 255, 0), 2)

#         # Draw landmark points
#         for point in scaled_landmarks:
#             cv2.circle(annotated_image, point, 5, (0, 0, 255), -1)

#     return annotated_image
