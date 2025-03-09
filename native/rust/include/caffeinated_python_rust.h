#ifndef RUST_H
#define RUST_H

#ifdef __cplusplus
#define EXPORT extern "C"
#else
#define EXPORT
#endif

EXPORT void test();

#endif