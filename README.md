# rust_jwt_bench_v1

This repo is an answer to : 

https://github.com/pjgg/javaJwtSignExample/

(I compared my Rust code with your optimized Java code.)




@pjgg You've compared multithreaded java code against single-threaded Rust code, here's a multithreaded Rust code that doesn't use all available CPUs ( I've removed 2 CPUs to chalenge Rust even more ).
Here are the results + a video as proof :

    <video src="[URL](https://github.com/rxdiscovery/rust_jwt_bench_v1/blob/main/Screencast%202024-05-15%2005%3A22%3A01.mp4)">video</video>




## Rust MT Code

```
=== Rust bench ===
total cpus : 24
use only : 22 cpus
Total iterations : 1000022 , endend in : 204 ms
```

Max Cores used : 22 / 24
Max CPU usage % : ~25%
Max Memory usage Mb : ~125 Mb 


## Java MT Code

```
=== Java bench ===
total cpus : 24
use only : 24 cpus
Total iterations : 1000024 , endend in : 7893 ms
```

Max Cores used : 24 / 24
Max CPU usage % : ~92%
Max Memory usage Mb : ~1150 Mb 


# Results



| Code    | Best latency (ms) | Max CPU usage (%) | Max Memory Usage (%) |
| -------- | ------- | ------- | ------- |
| Rust  | 204 ms   | ~25% | ~125 Mb |
| Java | 7893 ms    | ~92% | ~1150 Mb |


# Conclusion


Rust 3769% (38x) faster than Java in this case, and uses far fewer resources.
