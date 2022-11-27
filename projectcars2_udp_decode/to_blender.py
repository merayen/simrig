import bpy
import os
import json
from math import sin

fps = 60

fcurves = bpy.data.objects['Car'].animation_data.action.fcurves

# Delete all keyframes for the car
for fcurve in fcurves.values():
	while fcurve.keyframe_points:
		fcurve.keyframe_points.remove(fcurve.keyframe_points[0])

# Read data from file
with open("/media/merayen/Ting/Dokumenter/simrig/projectcars2_udp_decode/output.json") as f:
	data = json.load(f)

# Get the start time, as we probably have truncated it
start_time = data[0]["time"]

# Add keyframes
for fcurve in fcurves.values():
	if fcurve.data_path == "location":
		for i,d in enumerate(data):
			fcurve.keyframe_points.insert(i, d["location"][[0,2,1][fcurve.array_index]])
	if fcurve.data_path == "rotation_euler":
		for i,d in enumerate(data):
			fcurve.keyframe_points.insert(i, d["rotation"][[0,2,1][fcurve.array_index]])
