import ctypes
import pathlib
import os
import math


lib = ctypes.CDLL(pathlib.Path("target/release/libestimators.so"))
lib.central_difference_formula.restype = ctypes.c_double
lib.central_difference_formula.argtypes = [ctypes.CFUNCTYPE(ctypes.c_double, ctypes.c_double),
ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_size_t]

lib.generic_io_fn.restype = ctypes._types.NoneType()
lib.generic_io_fn.argtypes = []

def call_test():
    return lib.generic_io_fn()


def call_cdf(func, x_value, step_size, step_reduction_factor, iterations):
    LAMBDA_FACTORY = ctypes.CFUNCTYPE(ctypes.c_double, ctypes.c_double)
    function = LAMBDA_FACTORY(func)
    return lib.central_difference_formula(function, x_value, step_size, step_reduction_factor, iterations)


call_cdf(lambda x: math.exp(x), 1.0, 1.0, 0.1, 4)
#call_test()