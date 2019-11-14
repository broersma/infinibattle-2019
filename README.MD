# Neato

## Introduction

Neato is my 2019 entry into [Infi](https://www.infi.nl)'s yearly Infinibattle AI-bot competition. It did not do well, but might still be interesting to check out.

It uses [NEAT](http://www.cs.ucf.edu/~kstanley/neat.html) (using [python-neat](https://neat-python.readthedocs.io/)) to evolve a neural network that decides the bot's moves.

Neato (infinibattle-giuseppe) has to play against itself and against "The real Giuseppe" (infinibattle-giuseppe-real). Genomes which resulted in winning phenotype/neural network get awarded a higher fitness and are selected into the next generation.

The real Giuseppe is a simple bot which is added as an exploiter so evolution goes in the right direction.

## Usage

### Requirements
- Python 3.x
- Rust 1.38+

### Configure

Configure python-neat in `infinibattle-giuseppe/config-feedforward`. Immediately interesting settings are:

- pop_size - Number of genomes per generation.
- no_fitness_termination - If true fitness_criterion is ignored.
- fitness_criterion + fitness_threshold - Immediately finish the evolution when a single genome achieves this. (Gotta be pretty sure about your fitness function's effectiveness, or else you might accidently end up with a winning genome who was just lucky for one generation.)
- num_inputs - Number of inputs, in case you want to modify this in `infinibattle-giuseppe/StarterBot.py`.
- num_outputs - Number of outputs, in case you want to modify this in `infinibattle-giuseppe/StarterBot.py`.

Check out [the docs](https://neat-python.readthedocs.io/en/latest/config_file.html) for more information.

Adjust the maximum number of generations in `infinibattle-giuseppe/evolve-minimal.py` at this line:
```python
winner = p.run(eval_genomes, 100)
```

### Evolve

To run evolution:

```
$ cd infinibattle-giuseppe
$ python3 evolve-minimal.py 
```
	
This should result in two files `the_config.dump` and `the_genome.dump`, containing a pickled configuration based on `infinibattle-giuseppe/config-feedforward` and the winning genome, respectively.

If `python3 StarterBot.py` is ran without arguments it automatically attempts to load those two files.

## Publish

```
$ cd infinibattle-giuseppe
$ # *add your APIKEY to the Makefile*
$ make publish
```
