#!/usr/bin/env python3
"""Generates tests/fixtures/placement_identity.onnx — a real, minimal ONNX
model (Relu over a dynamic-length [1, len] float input) used to exercise the
tract-ONNX branch of PlacementCNN end-to-end. It is a fixture, not a trained
placement model: output scores equal relu(input), so placements are simply
the frames scoring >= threshold. Regenerate: python scripts/make_onnx_fixture.py
"""
import os
import onnx
from onnx import TensorProto, helper

def main():
    here = os.path.dirname(os.path.abspath(__file__))
    out = os.path.join(here, "..", "tests", "fixtures", "placement_identity.onnx")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    x = helper.make_tensor_value_info("X", TensorProto.FLOAT, [1, "len"])
    y = helper.make_tensor_value_info("Y", TensorProto.FLOAT, [1, "len"])
    node = helper.make_node("Relu", inputs=["X"], outputs=["Y"])
    graph = helper.make_graph([node], "placement_identity", [x], [y])
    model = helper.make_model(graph, opset_imports=[helper.make_opsetid("", 13)])
    model.ir_version = 8
    onnx.checker.check_model(model)
    onnx.save(model, out)
    print(f"wrote {out} ({os.path.getsize(out)} bytes)")

if __name__ == "__main__":
    main()
