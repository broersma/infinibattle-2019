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

def select_target_and_power(planet, targets):
    target = min(targets, key=lambda target: target.squared_distance_to(planet))
    power = planet.health - 1.000003
    return target, power

def strategy1(planets, ships):    
    # neutrale planeten opportuun pakken
    # last minute planeet pakken als tegenstander hem bijna heeft
    # cohesive blijven == slimmer reinforcen
    # alleen aanvallen als je een planeet kan pakken
    # reinforcen als je aangevallen wordt
    
    # enemies:
    # - HeartsAndMinds v4
    # - Unicorn v58
    # - Schaakmeester v26
    # - TheMole v19
    # - Armada v17
    # - TheFederation v36
    
    planets_by_id = {}
    for planet in planets:
        planets_by_id[planet.id] = planet

    my_planets = [planet for planet in planets if planet.owner == game_settings.my_player_id]
        
    other_planets = [planet for planet in planets if planet not in my_planets]
    

    frontline_planets = []
    frontline_planets.append(other_planets)
    
    while my_planets:
        frontline_planets.append([planet for planet in my_planets if any(planets_by_id[neighbor] in frontline_planets[-1] for neighbor in planet.neighbors)])
        my_planets = [planet for planet in my_planets if planet not in frontline_planets[-1]]
        

    for level in range(1, len(frontline_planets)):
        for planet in frontline_planets[level]:
            if planet.health > 2.0:
                targets = [neighbor for neighbor in planet.neighbors if planets_by_id[neighbor] in frontline_planets[level-1]]
            
                if targets:
                    target, power = select_target_and_power(planet, [planets_by_id[target_id] for target_id in targets])
                    print(f"send-ship {power} {planet.id} {target.id}")

if __name__ == '__main__':
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
        
        strategy1(planets, ships)

        print('end-turn')
