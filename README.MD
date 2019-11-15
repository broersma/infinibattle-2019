# Neato

## Introduction

Neato is my 2019 entry into [Infi](https://www.infi.nl)'s yearly Infinibattle AI-bot competition. It did not do well, but might still be interesting to check out.

It uses [NEAT](http://www.cs.ucf.edu/~kstanley/neat.html) (using [python-neat](https://neat-python.readthedocs.io/)) to evolve a neural network that decides the bot's moves.

Neato (infinibattle-giuseppe) has to play against itself and against "The real Giuseppe" (infinibattle-giuseppe-real). Genomes which resulted in winning phenotype/neural network get awarded a higher fitness and are selected into the next generation.

"The real Giuseppe" is a simple bot which is added as an "exploiter"[1] so evolution goes in the right direction.

*[1] Not sure if this is an actual exploiter, I just read about it in the context of AlphaStar, and it sounded cool so now I'm using it too.*

## Background

### Implementation

The brain of the bot can be found in the `strategy2` function in `infinibattle-giuseppe/StarterBot.py`.

In this function the neural network is applied, for every owned planet, to every pair of the owned planet and each of its neighbour planets. It has 4 inputs:

- planet health
- neighbour planet health - positive if it is owned by the bot, otherwise negative
- neighbour planet's friendly neighbour health - sum of healths of all planets owned by the bot and neighbouring the neighbour
- neighbour planet's enemy neighbour health - sum of healths of all planets not owned by the bot and neighbouring the neighbour

The neural network has 1 output which is interpreted as a weighting for allocating outgoing ships from the owned planet in question. If the net yields a positive output for a specific neighbor this is interpreted as the planet wanting to send ships to this neighbor (either to reinforce or attack). A negative output is interpreted as wanting to keep ships at the planet.

### Possible improvements

- I'm not quite sure if it is possible to get negative outputs. It sure seemed to always want to send out ships to all neighbours.  This would need to be tested.
- The neural network is quite simple and doesn't take global patterns in to account. A big obstacle I found is finding some way to map the game state to a neural network with a fixed number of inputs. It might be interesting to find a way to combine Graph Neural Networks[2], which take graphs as input, with NEAT. I attempted to implement PATCHY-SAN in the `strategy3` function in `infinibattle-giuseppe/StarterBot.py`, but I didn't get any results quick enough so for the Infinibattle I put my money on the implementation outlined above.
- More processing power: running a lot more generations and having a bigger populations.
- Improving the fitness function. The fitness function is now defined as the per-match-average game score (planet+ship health / total planet+ship health) in a full competition between a generation's genomes combined with an instance of "The real Giuseppe". This means the fitness function is optimal when a bot accidentally wins all matches against its opponents within a generation, however, a theoretically optimal fitness function would only reach its maximum for a bot that displays global optimal play. It might be better to mix in other variables into the fitness function like total rounds played (which we probably would like to minimize).
- Tweaking the python-neat configuration to better fit the neural network we want to create.

*[2] Graph Neural Networks are a generalization of Convolutional Neural Networks popularly used for image processing. This article seems a good introduction to Graph Neural Networks: [A Comprehensive Survey on Graph NeuralNetworks](https://arxiv.org/pdf/1901.00596.pdf).*

## Usage

### Requirements
- Python 3.x
- Rust 1.38+

### Configure

Configure python-neat in `infinibattle-giuseppe/config-feedforward`. Immediately interesting settings are:

- `pop_size` - Number of genomes per generation.
- `no_fitness_termination` - If true, `fitness_criterion` is ignored.
- `fitness_criterion` + `fitness_threshold` - Immediately finish the evolution when a single genome achieves this. (Gotta be pretty sure about your fitness function's effectiveness, or else you might accidently end up with a winning genome who was just lucky for one generation.)
- `num_inputs` - Number of inputs, in case you want to modify this in `infinibattle-giuseppe/StarterBot.py`.
- `num_outputs` - Number of outputs, in case you want to modify this in `infinibattle-giuseppe/StarterBot.py`.

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
