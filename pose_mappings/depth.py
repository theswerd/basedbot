
from transformers import pipeline
from PIL import Image
import time
import numpy as np

# pipe = pipeline(task="depth-estimation",
#                 model="depth-anything/Depth-Anything-V2-Metric-Indoor-Small-hf")
# image_path = '/Users/jchunx/code/k1/ml-depth-pro/data/example.jpg'
# image = Image.open(image_path)
# PYTORCH_ENABLE_MPS_FALLBACK = 1

# Profile the pipeline
# num_runs = 5
# times = []
# for _ in range(num_runs):
#     start = time.time()
#     _ = pipe(image)
#     end = time.time()
#     times.append(end - start)

# avg_time = np.mean(times)
# std_time = np.std(times)

# print(
#     f"Average inference time over {num_runs} runs: {avg_time:.3f}s ± {std_time:.3f}s")


class DepthModel:
    def __init__(self):
        self.pipe = pipeline(task="depth-estimation",
                             model="depth-anything/Depth-Anything-V2-Metric-Indoor-Small-hf")

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
        depth_array = np.array(depth_output["depth"])

        return depth_array


# # Example usage
# depth_map = pred_depth(image)
# Image.fromarray(depth_map).save("depth_map.png")
