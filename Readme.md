# Evolution of parental instict
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

I should note,  World sends all the agents to Decisions at once, then Decisions to Move, and then move applied them all.
## Rules

### Background
Our blips live on a rogue planet, drifting through interstellar space, mostly undetectable to human beings. They live underwater on the sea floor.
Their planets tectonic movement, combined with the gravitational energy provided by their world's interaction with her sister plannet, create just 
enough thermal energy to allow for liquid water underneath the vast expanse of ice that envelops her surface. A barren, star-less corner of the 
universe. The blips cannot look forward to the promise of spring in the cold entrails of winter, as neither exist. There are no seasons
to speak of. No stars to be seen during the nighttime, as day and night hold little meaning on this world. Not that they would be visible anyways 
through the ice that is their sky. Our blips have evolved to be constantly alert underwater, as the dangers of sleeping in this environment is immense.
Though they are quite different from any life you would find on earth, many similarities remain. Similar central nervous system as found in most
intelligent species; a face that resembles that of a blobfish on land; and, crucially, sexual production. Involving a male, and female species.

Huzzah evolution!

### Time scales
30 seconds = 1 Blip day (their planet spins really fast, probably why they have to live on the sea floor)
Mating season = 10 Blip days = 5 minutes
Off season = 30 Blip days = 10 minutes
1 Year = 40 Blip days = 15 minutes

All blips die at 5 years old (1 hour)

### Genetics
Inspired by the research papers linked above:

**effort = 0.4(father) + 0.4(mother) + 0.2(random about normal distribution)**
This represents the amount of effort that a child might put into its own children.

**reproductive_potential = 0.3(father) + 0.3(mother) + 0.4(random)**
Randomness is weighted highly here to increase variability and hopefully skew the results of runs into more or less overall
parental care

**attentional_resources = 0.33(father) + 0.33(mother) + 0.33(random about normal distribution)**


### Mating Season
Blips will be on the hunt for a midnight liason for the duration of the mating season. There are, of course, male and female blips. 
For simplicity's sake, we won't worry much about the mating mechanisms. We'll just say that every encounter between a male and a female has a 10% chance of spawning a child.
This number is completely arbitrary. I simply believed that it would be high enough to not kill our blips and low enough to not give every single blip a child every single time.

Each mating encounter will generate 3 children.1 above replacement, to account for blips who didn't find a mate that season. 

Every blip is eligible to mate after having reached 3 years old.

### Feeding
Food spawns at random locations every 5 blip days or so. Blips will retain memory of the food locations, sorting by distance and looking for them 
once they get hungry. Each blip will have an active energy meter that will determine whether or not to loog for food. The effort and 
attentional_resources genes will determine things like whether the blip feeds their children or themselves first, and if so, how many children they 
feed before feeding themselves. 

Children will stay in their nests to be fed, but venture out to find food once thieir hunger reaches a threshold.

**hunger_threshold = max(reproductive_potential^2, 0.1)**
Blips with a higher reproductive potential are more resilient, and more likely to venture out into the wild.

### Predation
Layered on top of our blips' world is a predation heat map. This will be included as a part of the simulator's spatial partitioning.

The likeliness of dying in a given location will be:
**death_chance = max(heat * cos^2(π/2 * energy), 0.3)**

The heat of any given partition will be between 0 and 1. If a blip has a full energy bar, their chances of predation goes to 0, 
as the assumption is that they had enough energy to run away. Conversely, if their energy bar is empty they'll already be dead, but if 
they weren't, their chances of predation is exactly equal to the heat of the location.

### Traits and States
TBC

Remember to make Energy tied to hunger