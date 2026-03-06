# Evolution of parental instinct
Whether viewed through the lens of science or history, Darwinism's impact on modern biology has been undeniably profound.
Yet, for something so important, it is incredibly hard to visualize; the time scales over which evolution operates are simply unfathomable to
the human mind.  

Learning about evolution's many answers to natures most simple and foundational questions - like "how do I feed?" and "how do I 
breathe?" - has always fed a desire inside me to be able to witness how these answers came to be. Thanks to my newfound simulating superpowers,
now I can! As such, the next step in my simulation journey shall be an attempt to figure out how come evolution has created so many ways
to deal with nature's most foundational principle: "Protect my child."

In all actuality, this is not going to be the most accurate evolution simulator ever created; there are people who dedicate their
lives to doing this. That being said, it won't be completely uninformed. This project was built to satisfy my own intellectual curiosity,
and thus would have little value based entirely upon guesswork. If you're interested in seeing the sources I used to build it, I invite
you to explore them:

[Parental Investment Contingency Model](https://www.sciencedirect.com/science/article/abs/pii/S109051380800007X)   
[Heritability of Parental Effort in a Passerine Bird (2003)](https://pubmed.ncbi.nlm.nih.gov/14575340/)

Those were pretty long reads, so here are the most relevant takeaways:
1. Parents' attentional resources were affected by both the amount of said resource and the reproductive potential of the child
2. Less Resources meant children with more reproductive potential received more of those resources. More resources meant the opposite.
3. The more parents invest in their children the more likely they are to survive (obviously)
4. Parental effort is an inherited trait

In the interest of my relentless battle against mundanity in these simulations, I also considered some of the less relevant parts of the papers:
1. [Added from papers] Adults try to find a mate, then if they can't (poor guys) they help raise the offspring of their siblings
2. [Removed from papers] Parental effort does not affect parent survival (I added a parental effort tax)

I believe these modifications were good for adding some extra variability and raising the stakes.

## System architecture
The system follows a modular, top-down execution flow, with World serving as the primary orchestrator and container of blips.
It is set up in such a way to allow changes to cascade from most abstract to most granular, involving all agents at the same time.
This makes for a much more straightforward implementation of things like reproduction and spatial partitioning. 

```
       -----------------          -----------------------
       |     World     | -------> |    Visualization    |
       |   ---------   |          -----------------------
    ---|-> | Blip  |   | -------> |   Data Aggregation  |
    |  |   ---------   |          -----------------------
    |  |       |       |                     ^
    |  |        -------|---------------------|
    |  -----------------                     |
    |          |                             |
    |          V                             |
    |      Decisions                         |
    |          |                             |
    |    --------------                      |
    |    |            |                      |
    |    V            V                      |
    --- Move    Reproduction -----------------
```
### World [The... well the world]
- Houses
    - Blips
    - Environment (e.g predation heat map, locations of food, spatial partitioning)
    - Publishes events to update visualization and Data Aggregation

### Blip [My personal perfectly fictional entity]
- Only agent type in this simulation
- Houses Blip and information to do with identity
    - e.g. Mother, Father, Age, Relevant Genes

### Decisions [The Brain]
- Makes decisions on blip's behalf
    - e.g. find food, mate
- If necessary
  - Updates blip's target location
  - Sends parent information to Reproduction

### Reproduction [The Baby-making Factory!!]
- Makes new blips and sends them to world

### Move [The Brawn]
- Literally just moves the blip towards its target
- That's it.
- This doesn't have to be complicated. Move on to the next section.

### Visualization and Data Aggregation [The supporters]
- Do exactly what it sounds like they do

I should note,  World divides all blips via spatial-partitioning, and passes each partition into the Decisions to engage the execution flow.
At the end, of one cycle, world applies the changes to the blips, based on the state of the world (e.g., adding a newborn blip / killing a blip 
by way of the predation heat map)
## Rules

### Lore
Our blips live on a rogue planet, drifting through interstellar space, mostly undetectable to human beings. They live underwater on the sea floor.
Their planets tectonic movement, combined with the gravitational energy provided by their world's interaction with her sister planet, create just 
enough thermal energy to allow for liquid water underneath the vast expanse of ice that envelops her surface. A barren, star-less corner of the 
universe. The blips cannot look forward to the promise of spring in the cold entrails of winter, as neither exist. There are no seasons
to speak of. No stars to be seen during the nighttime, as day and night hold little meaning on this world. Not that they would be visible anyway 
through the ice that is their sky. Our blips have evolved to be constantly alert underwater, as the dangers of sleeping in this environment is immense.
Though they are quite different from any life you would find on earth, many similarities remain. Similar central nervous system as found in most
intelligent species; a face that resembles that of a blobfish on land; and, crucially, sexual production. Involving a male, and female species.

Huzzah evolution!

### Time scales
30 seconds = 1 Blip day (their planet spins really fast, which is why they have to live on the sea floor)  
Mating season = 10 Blip days = 5 minutes  
Off season = 30 Blip days = 10 minutes  
1 Year = 40 Blip days = 15 minutes

All blips die at 5 years old (1 hour)

### Genetics
Inspired by the research papers linked above:

$$effort = \frac{2}{5}(father) + \frac{2}{5}(mother) + \frac{1}{5}(\text{random})$$
This represents the amount of effort that a child might put into its own children.

$$reproductive\ potential = \frac{3}{10}(father) + \frac{3}{10}(mother) + \frac{4}{10}(\text{random})$$
Randomness is weighted highly here to increase variability and hopefully skew the results of runs into more or less overall
parental care

$$attentional\ resources = \frac{1}{3}(father) + \frac{1}{3}(mother) + \frac{1}{3}(\text{random})$$
This just felt right. You were probably expecting some super thought out explanation, but that's literally it. 

All above genes are on a scale of 0-1. 

### Mating Season
Blips will be on the hunt for a midnight liason for the duration of the mating season. There are, of course, male and female blips. 
For simplicity's sake, we won't worry much about the mating mechanisms. We'll just say that every encounter between a male and a female has a 10% chance of spawning a child.
This number is completely arbitrary. I simply believed that it would be high enough to not kill our blips and low enough to not give every single blip a child every single time.

During mating season, blips are only ever either looking for food or getting down.

Each mating encounter will generate 3 children.1 above replacement, to account for blips who didn't find a mate that season. 

Every blip is eligible to mate after having reached 3 years old.

### Feeding
Food spawns at random locations every 5 blip days or so. Blips will retain memory of the food locations, sorting by distance and looking for them 
once they get hungry. Each blip will have an active energy meter that will determine whether to look for food. The effort and 
attentional_resources genes will determine things like whether the blip feeds their children or themselves first, and if so, how many children they 
feed before feeding themselves. 

Children will stay in their nests to be fed, but venture out to find food once their hunger reaches a threshold.

$$hunger\_threshold = \max(reproductive\_potential^{2}, 0.1)$$
Blips with a higher reproductive potential are more resilient, and more likely to venture out into the wild.

### Predation
Layered on top of our blips' world is a predation heat map. This will be included as a part of the simulator's spatial partitioning.

The likeliness of dying in a given location will be:  
$$death\_chance = heat \cdot \cos^{2}\left(\frac{\pi}{2} \cdot energy\right)$$

The heat of any given partition will be between 0 and 1. If a blip has a full energy bar, their chances of predation goes to 0, 
as the assumption is that they had enough energy to run away. Conversely, if their energy bar is empty they'll already be dead, but if 
they weren't, their chances of predation is exactly equal to the heat of the location.

effort will be the value used to determine how much risk a blip is comfortable taking. a 0.7 effort value would mean that a 
blip is comfortable traveling into partitions with up to a 0.7 heat. This is my "parental effort tax". Blips who put in more effort
also expose themselves to predation. This also means children are more bold, but children only venture out of nests if they have neglectful parents. 
Given the normal distribution of randomness, blips are unlikely to deviate much from their parents when it comes to the effort gene. This means that
They will also not be very willing to take risks if their parents neglect them, and more willing to if their parents don't. Then again, if their parents
take good care of them, they are less likely to have to venture beyond the nest. All in all, I doubt it will become a serious issue.

### Traits, States, and Objectives
#### States
- **hunger** - A little counter-intuitive, this will actually be the _lack_ of hunger. A higher number means less hunger, a lower number means more hunger. 
0 hunger will start eating away at energy, while full hunger (1) will give energy a boost.  
- **energy** - 0-1;  Dictates the amount of energy a blip has at any given moment. Energy expenditure increases hunger.  
- **position, velocity, acceleration** - States to track physics-based movement and location on map  
- **target** - Blip's desired destination at any moment in time
- **objective** - One of the below objectives
- **Happiness** - This one is a little odd, I admit.  IT's a measure of how much attentional resources the child is receiving. If the happiness gets 
too low, it starts to negatively affect energy and hunger. 

#### Traits
- **genes** - These had a whole section  
- **father, mother** - Keeping track of parents allows blips to figure out who they're related to, and gives
us the option of tracking the progression of a gene over generations  
- **children_birthed** - A list of direct children  
- **children_raised** - A list of all children raised, including those of kin  
- **sex** - Blip gender... get your head out of the gutter  

#### Objectives
1. Make Child
2. Raise Child
3. Find Food (Bonus, can exist alongside other two)

When raising children, if the parents aren't finding food, they are spending attentional resources on the children.

# Technologies/Methods Used
1. Python
2. Rust
3. JavaScript
3. Numpy
4. Agent-Based Simulation
5. p5.js
6. React
5. Genetic Algorithms
6. Continuous spatial coordinates
7. Spatial Partitioning

# Phases
## Completed

## In progress
1. Basic agent movement (with spatial partitioning)

## Planned
1. Agent reproduction
2. Agent objective/target setting
3. Targeted Agent movement and collision avoidance
4. Agent behavioural patterns (based on genetics)
