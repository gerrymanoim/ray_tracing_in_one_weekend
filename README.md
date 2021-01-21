# Ray Tracing In One Weekend

Rust version of https://raytracing.github.io/books/RayTracingInOneWeekend.html

## Current Image Status

Using `convert image.ppm imgage.png`.

![](https://raw.githubusercontent.com/gerrymanoim/ray_tracing_in_one_weekend/main/image.png)


## Random Notes

Surface normals are vectors perpendicular to to the surface at the point that we hit the surface. On earth, in your present location that points straight up.

![](https://raytracing.github.io/images/fig-1.05-sphere-normal.jpg)

Here are normals are always outside normals, i.e. they face outward independently of whether the ray is coming from inside or outside.

![](https://raytracing.github.io/images/fig-1.06-normal-sides.jpg)
