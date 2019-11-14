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

Adjust the number of iterations in evolve-minimal.py at this line:
```python
winner = p.run(eval_genomes, 100)
```

### Evolve

To run evolution:

```
$ cd infinibattle-giuseppe
$ python3 evolve-minimal.py 
```
	
This should result in two files 'the_config.dump' and 'the_genome.dump'. If StarterBot.py is ran without arguments these files are automatically loaded.

## Publish

```
$ cd infinibattle-giuseppe
$ # *add your APIKEY to the Makefile*
$ make publish
```
