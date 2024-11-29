:- use_module(library(clpfd)).
:- use_module(library(assoc)).
:- use_module(library(yaml)).
% :- set_prolog_flag(double_quotes, chars).
%  
%
geosProblem(X) :- 
  yaml_read(string("

solvers:
  - solver: ReactiveCompositionalMultiPhaseOBL
    name: compflow


"), X).

pressure_unit(X) :- member(X, ['bar', 'Pa']).
pressure(Value, Unit) :- number(Value), pressure_unit(Unit).

compositional_solver(geos, 'ReactiveCompositionalMultiPhaseOBL').
compositional_solver(darts, 'super').

geothermal_solver(darts, 'geothermal').

solver(Framework, Solver) :- 
    geothermal_solver(Framework, Solver) ;
    compositional_solver(Framework, Solver).

geode_to_geos(A, B) :- A = B.

avion(airbus).
avion(boeing).


:- forall(geosProblem(X), (write(X), nl)).
