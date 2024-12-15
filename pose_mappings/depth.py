import os
os.environ["PYTORCH_ENABLE_MPS_FALLBACK"]="1"
from transformers import pipeline
from PIL import Image
import time
import torch
import numpy as np

if torch.cuda.is_available():
    _device = 'cuda:0'
else:
    _device = 'mps'

class DepthModel:
    def __init__(self):
        self.pipe = pipeline(task="depth-estimation", device=_device,
                             model="depth-anything/Depth-Anything-V2-Metric-Indoor-Large-hf")

    def pred_depth(self, image):
        """
        Predict depth map from a PIL Image.

        Args:
            image (PIL.Image): Input image

        Returns:
            np.ndarray: Depth map as numpy array
        """
        # Get depth prediction
        depth_output = self.pipe(image)

        # Convert PIL Image depth map to numpy array
        depth_array = np.array(depth_output["depth"]) / 100.0

        return depth_array
    

