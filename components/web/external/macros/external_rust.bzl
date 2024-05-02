# @nolint

def external__rust_cxx_library(name, **kwargs):
    native.cxx_library(name = name, **kwargs)

def external__rust_prebuilt_cxx_library(name, **kwargs):
    native.prebuilt_cxx_library(name = name, **kwargs)
