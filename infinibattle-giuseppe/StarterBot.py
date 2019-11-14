def read_value(key):
    line = input()
    parts = line.split()
    if len(parts) != 2 or parts[0] != key:
        raise Exception("Expected '%s' <value>', got '%s" % (key, line))
    return parts[1]

def read_owner(owner):
    if owner == 'neutral':
        return None
    return int(owner)

class Planet:
    def __init__(self, parts, neighbors):
        self.id = int(parts[1])
        self.x = float(parts[2])
        self.y = float(parts[3])
        self.radius = float(parts[4])
        self.owner = read_owner(parts[5])
        self.health = float(parts[6])
        self.neighbors = neighbors
        
    def squared_distance_to(self, planet):
        dx = abs(self.x - planet.x)
        dy = abs(self.y - planet.y)
        return dx*dx + dy*dy
        
    def __repr__(self):
        return f"\n# {self.id} o={self.owner} hp={self.health}"

def read_neighbors():
    line = input()
    parts = line.split()
    if len(parts) == 0 or parts[0] != 'neighbors':
        raise Exception("Expected 'neighbors <neighbor1> <neighbor2> ...', got '%s'" % line)
    return [int(x) for x in parts[1:]]

def read_planets():
    def read_planet():
        line = input()
        parts = line.split()
        if len(parts) != 7 or parts[0] != 'planet':
            raise Exception ("Expected 'planet <id> <x> <y> <radius> <owner> <health>'', got '%s" % line)
        return Planet(parts, read_neighbors())

    planet_count = int(read_value('num-planets'))
    return [read_planet() for i in range(planet_count)]

class Ship:
    def __init__(self, parts):
        self.x = float(parts[1])
        self.y = float(parts[2])
        self.target_id = int(parts[3])
        self.owner = read_owner(parts[4])
        self.power = float(parts[5])

def read_ships():
    def read_ship():
        line = input()
        parts = line.split()
        if len(parts) != 6 or parts[0] != 'ship':
            raise Exception("Expected 'ship <x> <y> <target_id> <owner> <power>', got '%s'" % line)
        pass

    ship_count = int(read_value('num-ships'))
    return [read_ship() for i in range(ship_count)]

class GameSettings:
    def __init__(self):
        self.seed = int(read_value('seed'))
        self.num_players = int(read_value('num-players'))
        self.my_player_id = int(read_value('player-id'))

game_settings = GameSettings()


def strategy2(planets, ships, net):
    
    planets_by_id = {}
    for planet in planets:
        planets_by_id[planet.id] = planet

    my_planets = [planet for planet in planets if planet.owner == game_settings.my_player_id]
    
    for planet in my_planets:
        if planet.health > 2.0:
            neighbor_output = {}
            for neighbor_id in planet.neighbors:         
                other_planet = planets_by_id[neighbor_id]
                
                other_planet_neighbor_health = {}
                other_planet_neighbor_health[True] = 0.0
                other_planet_neighbor_health[False] = 0.0
                for other_planet_neighbor_id in other_planet.neighbors:
                    if other_planet_neighbor_id != planet.id:
                        other_planet_neighbor = planets_by_id[neighbor_id]
                        friendly = other_planet_neighbor.owner == game_settings.my_player_id
                        other_planet_neighbor_health[friendly] += other_planet_neighbor.health

                input = (planet.health,
                         other_planet.health if other_planet.owner and other_planet.owner == game_settings.my_player_id else -other_planet.health,
                         other_planet_neighbor_health[True],
                         other_planet_neighbor_health[False],)
                output = net.activate(input)
                neighbor_output[neighbor_id] = output[0]

            available_power = planet.health - 1.0001            
            sum_values = sum(abs(value) for value in neighbor_output.values())
            if sum_values > 0:
                available_power_per_diff = available_power / sum_values
                for neighbor_id in neighbor_output:
                    power = available_power_per_diff * neighbor_output[neighbor_id]
                    if power > 1:
                        print(f"send-ship {power} {planet.id} {neighbor_id}")
                        
def strategy3(planets, ships, net):
    
    planets_by_id = {}
    for planet in planets:
        planets_by_id[planet.id] = planet

    my_planets = [planet for planet in planets if planet.owner == game_settings.my_player_id]
    
    planet_output = {}
    for planet in planets:
        neighbors_radius = defaultdict(list)
        neighbors_health = defaultdict(list)
        
        neighbor_planets_by_distance = sorted((planets_by_id[neighbor_id] for neighbor_id in planet.neighbors), key=lambda neighbor_planet: planet.squared_distance_to(neighbor_planet))
        
        num_neighbors_to_consider = 4 # multiply by num_neighbor_planet_inputs (currently 3) and add to num_my_planet_inputs (currently 2)
        
        input = (
                    planet.radius,
                    planet.health,
                )
        num_my_planet_inputs = len(input)
        for neighbor_planet in neighbor_planets_by_distance[:num_neighbors_to_consider]:
            # TODO: Add exploiters.
            
            neighbor_planet_inputs = (
                                        planet.squared_distance_to(neighbor_planet),
                                        neighbor_planet.radius,
                                        neighbor_planet.health if neighbor_planet.owner == game_settings.my_player_id else -neighbor_planet.health,
                                     )
            
            num_neighbor_planet_inputs = len(neighbor_planet_inputs)            
            
            input += neighbor_planet_inputs
        
        # Pad with dummy inputs.
        current_total_neighbor_planet_inputs = int((len(input)-num_my_planet_inputs)/num_neighbor_planet_inputs)
        num_missing_neighbor_planet_inputs = num_neighbors_to_consider - current_total_neighbor_planet_inputs    
        input += (999, 0, 0,)*num_missing_neighbor_planet_inputs
        
        output = net.activate(input)
        #print('##########',planet.id)
        #print('##########',output)

        planet_output[planet.id] = output[0]
                
    for planet in my_planets:
        if planet.health > 2.0:
            #print('##########',planet.id)
            neighbor_diff = {}
            for neighbor_planet in (planets_by_id[neighbor_id] for neighbor_id in planet.neighbors):
                neighbor_diff[neighbor_planet.id] = planet_output[neighbor_planet.id] - planet_output[planet.id]
            
            #print('#',neighbor_diff)
            
            available_power = planet.health - 1.0001
            sum_values = sum(abs(value) for value in neighbor_diff.values())
            #print('#',neighbor_diff)
            
            if sum_values > 0:
                available_power_per_diff = available_power / sum_values
                
                for neighbor_id in neighbor_diff:
                    power = available_power_per_diff * neighbor_diff[neighbor_id]
                    if power > 0:
                        print(f"send-ship {power} {planet.id} {neighbor_id}")


if __name__ == '__main__':
    import sys
    import pickle
    import neat
    from collections import defaultdict
    
    if len(sys.argv) > 2:
        genome_filename = sys.argv[1]
        config_filename = sys.argv[2]
    else:
        genome_filename = 'the_genome.dump'
        config_filename = 'the_config.dump'

    with open(genome_filename, 'rb') as f:
        genome = pickle.load(f)
    with open(config_filename, 'rb') as f:
        config = pickle.load(f)
        
    net = neat.nn.FeedForwardNetwork.create(genome, config)

    while True:
        line = input()
        if line == 'game-end':
            break
        if line != 'turn-init':
            raise Exception("Expected 'turn-init', got '%s'" % line)

        planets = read_planets()
        ships = read_ships()

        line = input()
        if line != 'turn-start':
            raise Exception("Expected 'turn-start', got '%s'" % line)
        
        strategy2(planets, ships, net)

        print('end-turn')
