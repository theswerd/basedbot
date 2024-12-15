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