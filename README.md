# GEODE
A human first geoscience config file format, with CLI tools to generate input decks for various simulation frameworks (GEOS, DARTS). 

## Example
```yaml
# model or problem
physics:
  compositional:
    components:
      CO2:
        mass: 44.01 g/mol
      C1:
        mass: 16.04 g/mol
      H20:
        mass: 18.015 g/mol
    phases:
      - gas
      - oil
    constant_temperature: 1 °C

injectors:
   inj1:
      cuboid:
        x: -0.2 .. 20.2 m
        y: -0.2 .. 20.2 m 
        z: -0.2 .. 20.2 m

      pressure: 140 bar
      temperature: 74.85 °C
      components:
        C02: 0.99999998 
        C1:  1e-8
        H20: 1e-8 

producers:
  prd1:
    cuboid:
      x: 479.8 .. 500.2 m
      y: 479.8 .. 500.2 m
      z: -0.1999999993 .. 20.2 m

    pressure: 50 bar

domain:
  cuboid:
    x: 0 .. 500 m
    y: 0 .. 500 m
    z: 0 .. 20 m 

  materials:
    - rock
    - fluid

materials:
  rock:
    compressibility: 1e-09
    porosity: reference 0.3
    pressure: reference 5 000 000
    permeability:
      xx: 9.869e-14
      yy: 9.869e-14
      zz: 9.869e-14
  fluid:

conditions:
  initial:
    pressure: 50 bars
    temperature: 348.15
    globalComponentsFraction:
      CO2: 0.1
      C1: 0.2 
      H20: 0.7

  boundary:


# simulation
events:
  maxTime: 1 year
  simulation:
    start:
      dt: 1 day 
    1 week:
      dt: 2 weeks 
    end: 1 year
  vtkOutput:
    plotFileRoot: 2ph3c_square_static_geos


```

