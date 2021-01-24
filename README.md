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

Material names:
    - lambertian: matte
    - dielectrics: Clear materials such as water, glass, and diamonds
    -

Camera geometry

![](https://raytracing.github.io/images/fig-1.14-cam-view-geom.jpg)

And we also care about the orientation of the thing

![](https://raytracing.github.io/images/fig-1.15-cam-view-dir.jpg)

![](https://raytracing.github.io/images/fig-1.16-cam-view-up.jpg)

`(u, v, w)` describes the camera orientation in space


## TODO

- [ ] Fix usage of arc
- [ ] Refactor unit vector and refrecat - some things take values and some things take references
- [ ]
