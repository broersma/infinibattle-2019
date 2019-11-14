"""
2-input XOR example -- this is most likely the simplest possible example.
"""

import neat
import re
import os
import pickle
from itertools import combinations
import random
import string
import subprocess


def play_match(genome_a_id, genome_a_filename, genome_b_id, genome_b_filename):
    """
    This function will be run in parallel by ParallelEvaluator.  It takes two
    arguments (a single genome and the genome class configuration data) and
    should return one float (that genome's fitness).

    Note that this function needs to be in module scope for multiprocessing.Pool
    (which is what ParallelEvaluator uses) to find it.  Because of this, make
    sure you check for __main__ before executing any code (as we do here in the
    last few lines in the file), otherwise you'll have made a fork bomb
    instead of a neuroevolution demo. :)
    """

    output_filename = os.path.abspath('output/' + ''.join(random.choice(string.ascii_letters) for _ in range(16)) + '.json')

    p = subprocess.Popen(["cargo","run", "--release", "--quiet","--",
        "../../infinibattle-giuseppe/StarterBot.py",
        "../../infinibattle-giuseppe/StarterBot.py",
        "--args1",genome_a_filename,config_dump_filename,";",
        "--args2",genome_b_filename,config_dump_filename,";",
        "-o",output_filename,
        ],
        cwd="../planet-wars/runner")

    return (p, genome_a_id, genome_b_id, output_filename)

def play_match_with_the_real_giuseppe(genome_a_id, genome_a_filename):

    output_filename = os.path.abspath('output/' + ''.join(random.choice(string.ascii_letters) for _ in range(16)) + '.json')

    p = subprocess.Popen(["cargo","run", "--release", "--quiet","--",
        "../../infinibattle-giuseppe/StarterBot.py",
        "../../infinibattle-giuseppe-real/StarterBot.py",
        "--args1",genome_a_filename,config_dump_filename,";",
        "--args2","",";",
        "-o",output_filename,
        ],
        cwd="../planet-wars/runner")

    return (p, genome_a_id, None, output_filename)
    

def await_scores(match):
    (p, genome_a_id, genome_b_id, output_filename) = match
    
    p.wait()

    score_a = 0.5
    score_b = 0.5

    pattern = re.compile("\"score_bot1\":([0-9]+\.[0-9]+),\"score_bot2\":([0-9]+\.[0-9]+)")
    with open(output_filename, 'r') as f:
        for line in f:
            for match in re.finditer(pattern, line):
                score_a = float(match.group(1))
                score_b = float(match.group(2))
                break
            else:
                print("ERROR!!!\n" * 3)
                print("Check score_bot1, score_bot2 in replay.json: ", output_filename)
                print("Returning 0.5, 0.5")
                return genome_a_id, score_a, genome_b_id, score_b
    
    if not genome_b_id:
        print("Bot played the real giuseppe:", genome_a_id, score_a, genome_b_id, score_b, output_filename)
    else:
        os.remove(output_filename)

    return genome_a_id, score_a, genome_b_id, score_b

def create_genome_dump(genome, prefix = ''):
    genome_filename = os.path.abspath('output/' + prefix + ''.join(random.choice(string.ascii_letters) for _ in range(16)) + '.genome')
    os.makedirs(os.path.dirname(genome_filename), exist_ok=True)
    with open(genome_filename, 'wb') as f:
        pickle.dump(genome, f)
    return genome_filename

def eval_genomes(genomes, config):
    scores = {}
    genome_by_id = {}
    genome_filename_by_id = {}
    for genome_id, genome in genomes:
        scores[genome_id] = 0.0
        genome_by_id[genome_id] = genome
        genome_filename_by_id[genome_id] = create_genome_dump(genome)
        print(genome_id, genome_filename_by_id[genome_id])

    matches = []

    num_games_played_by_id = {}
    possible_matches = list(combinations(genome_by_id, 2))
    random.shuffle(possible_matches)
    for genome_a_id, genome_b_id in possible_matches:
        # TODO: Remove True or and demand a specific number of games_played against other bots. > 3 seems good.
        if True or genome_a_id not in num_games_played_by_id or genome_b_id not in num_games_played_by_id or random.choice(range(4)) == 0:
            genome_a_filename = genome_filename_by_id[genome_a_id]
            genome_b_filename = genome_filename_by_id[genome_b_id]

            match = play_match(genome_a_id, genome_a_filename, genome_b_id, genome_b_filename)
            
            if genome_a_id not in num_games_played_by_id:
                num_games_played_by_id[genome_a_id] = 0
            
            if genome_b_id not in num_games_played_by_id:
                num_games_played_by_id[genome_b_id] = 0
                
            num_games_played_by_id[genome_a_id] += 1
            num_games_played_by_id[genome_b_id] += 1
            
            matches.append(match)

    for genome_a_id in genome_by_id:
        genome_a_filename = genome_filename_by_id[genome_a_id]

        match = play_match_with_the_real_giuseppe(genome_a_id, genome_a_filename)
        
        if genome_a_id not in num_games_played_by_id:
            num_games_played_by_id[genome_a_id] = 0

        num_games_played_by_id[genome_a_id] += 1
        
        matches.append(match)

    for match in matches:
        genome_a_id, score_a, genome_b_id, score_b = await_scores(match)

        scores[genome_a_id] += score_a
        if genome_b_id:
            scores[genome_b_id] += score_b

    for genome_id, genome in genomes:
        average_score = scores[genome_id] / num_games_played_by_id[genome_id]
        genome.fitness = average_score ** 2
        print(genome_id, genome.fitness, num_games_played_by_id[genome_id])

    best_genome_id, best_genome = max(genomes, key=lambda genome_pair: genome_pair[1].fitness)
    
    for genome_id, genome_filename in genome_filename_by_id.items():
        if genome_id == best_genome_id:
            print("Saving", best_genome_id,"for this generation as", genome_filename_by_id[best_genome_id])
        else:
            os.remove(genome_filename)


def run(config_file):
    # Load configuration.
    config = neat.Config(neat.DefaultGenome, neat.DefaultReproduction,
                         neat.DefaultSpeciesSet, neat.DefaultStagnation,
                         config_file)
                         
    with open(config_dump_filename, 'wb') as f:
        pickle.dump(config, f)

    # Create the population, which is the top-level object for a NEAT run.
    p = neat.Population(config)

    # Add a stdout reporter to show progress in the terminal.
    p.add_reporter(neat.StdOutReporter(False))

    # Run until a solution is found.
    winner = p.run(eval_genomes, 100)

    # Display the winning genome.
    print('\nBest genome:\n{!s}'.format(winner))
    
    with open('the_genome.dump', 'wb') as f:
        pickle.dump(winner, f)
    
if __name__ == '__main__':
    # Determine path to configuration file. This path manipulation is
    # here so that the script will run successfully regardless of the
    # current working directory.
    local_dir = os.path.dirname(__file__)
    config_path = os.path.join(local_dir, 'config-feedforward')
    config_dump_filename = os.path.join(local_dir, 'the_config.dump')
    run(config_path)
