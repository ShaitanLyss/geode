# Framwork input autogeneration

The idea is that from the generic conceptual model, we want to autogenerate a framework implementation suggestion.

That is to say : How to translate each property, each element to its counterpart in the target framework.
It isn't as easy as simply mapping properties one to one, because for a example a Geode domain with cuboid could
result in many fields for a GEOS XML (internal mesh, cellblocks, regions), but this also applies in the other direction,
for example several Geode material properties can result in one special constitutive material.

Therefore we need to find a way to specify a list of properties from both ends.

Now, we could hard code this autogeneration but simulation frameworks are always evolving and their APIs may be unstable.
Thus, we prefer basing the autogeneration on a configuration file. Let's design it.

## Autogeneration configuration

Several existing configuration formats are available (JSON, TOML, XML, Yaml...), let us run through their advanges.


- JSON
    - Powerful
    - Machine friendly (easy to parse)
- TOML
    - Limited
    - Human friendly for simple cases
- XML
    - Terrible
- Yaml
    - Powerful
    - Human friendly

With our use case of specifying multiple complex conditions on both ends and specifying nested elements and properties,
we need a powerful format like JSON or Yaml. Besides, these formats will only be parsed once and framework input
autogeneration is not a time critical task, so we can afford to lose some parsing time for the sake of human
friendliness. Therefore, we will go with Yaml.

## Yaml

Yaml boasts many features, but let's first focus on its data structures.

At the core of Yaml, you have scalars, maps and sequences. 

### Scalars

Scalars are simple one dimensional value, such as booleans, numbers, strings, dates...

### Maps

A map represents a complex elements with properties that are either scalars, sequences or maps themselves. It is written
as a list of key - values. For example, a dog would be represented as such :

```yaml
type: animal  # string scalar
paws: 4  # number scalar

owner:  # complex type
    name:    Lyss
    surname: Selene

nicknames:  # sequence 
    - goofy
    - doggo
```

### Sequences

Sequences represent lists of elements. All elements need to share the same indentation and start with a dash '-'.



## Using Yaml for framework input generation

So we have complex conditions and complex nested structures of elements and properties to combine and express in Yaml.
How are we going to approch this ?


First of all, how do we refer to a property of an element so it can be used as an assignement or as a condition ?

The logical most simplest way would be to use a dot seperated form. For instance, in Geode, events.MaxTime would refer
to the MaxTime property in Geode. But how does that work for sequences ?

```Geode
materials:
    - rock:
        porosity: 1
        pressure: reference 10 bar
    - water:
        viscosity: 2

```

Let's say I want to express that materials that have a porosity and a reference pressure are mapped to VaryingPressureSolid,
and materials with a viscosity to Fluid. How would that look in Yaml ?

```Geoffrey
materials as m:
    - m.porosity? && m.pressure?.isReference: VaryingPressureSolid

    - m.viscosity?: Fluid

```

























