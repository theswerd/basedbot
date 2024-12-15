# Define the kinematic chain for the left arm
import numpy as np
from ikpy.chain import Chain
from ikpy.link import URDFLink

left_arm_chain = Chain(name="left_arm", links=[
    URDFLink(name="left_shoulder_pitch", bounds=(-1.5707963, 1.5707963), translation_vector=[0, 0, 0], rotation=[0, 0, 1]),
    URDFLink(name="left_shoulder_yaw", bounds=(-1.5707963, 1.5707963), translation_vector=[0, 0.1, 0], rotation=[0, 1, 0]),
    URDFLink(name="left_elbow_yaw", bounds=(-1.5707963, 1.5707963), translation_vector=[0, 0.1, 0], rotation=[1, 0, 0]),
])

# Define the kinematic chain for the right arm
right_arm_chain = Chain(name="right_arm", links=[
    URDFLink(name="right_shoulder_pitch", bounds=(-1.5707963, 1.5707963), translation_vector=[0, 0, 0], rotation=[0, 0, 1]),
    URDFLink(name="right_shoulder_yaw", bounds=(-1.5707963, 1.5707963), translation_vector=[0, -0.1, 0], rotation=[0, 1, 0]),
    URDFLink(name="right_elbow_yaw", bounds=(-1.5707963, 1.5707963), translation_vector=[0, -0.1, 0], rotation=[1, 0, 0]),
])

def compute_arm_ik(target_position, chain):
    """
    Compute inverse kinematics for the arm chain.

    Args:
        target_position (list): Target 3D position [x, y, z].
        chain (Chain): Kinematic chain for the arm.

    Returns:
        list: Joint angles in radians to reach the target position.
    """
    target_frame = np.eye(4)
    target_frame[:3, 3] = target_position
    
    joint_angles = chain.inverse_kinematics(target_frame)
    return joint_angles
