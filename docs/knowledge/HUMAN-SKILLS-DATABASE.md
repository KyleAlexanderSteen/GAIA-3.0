# Human Skills Database — GAIA 2.0

> **Issue:** [#626](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/626)  
> **Epic:** [#620 — Knowledge & Intelligence Database System](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/620)  
> **Sibling:** [HUMAN-KNOWLEDGE-DATABASE.md](./HUMAN-KNOWLEDGE-DATABASE.md)  
> **Status:** Schema v1.0  
> **Last Updated:** 2026-09-21

---

## Overview

The Human Skills Database (HSD) is GAIA 2.0's structured taxonomy of every human-learnable skill. Where the Human Knowledge Database (HKD) captures *what humans know*, the HSD captures *what humans can do* — the applied, practiced, embodied dimension of human capability.

Skills are not knowledge. A surgeon may know anatomy perfectly and still lack the manual dexterity built through 10,000 hours of practice. The HSD models this distinction explicitly, tracking skills from beginner to mastery with proficiency levels, learning pathways, and assessment frameworks.

The HSD integrates directly with:
- **Mi-Memory Framework** ([#644](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/644)) — GAIAN personal skill profiles are stored here
- **Human Knowledge Database** — skills unlock when prerequisite knowledge domains are loaded
- **Human Superpowers Database** ([HUMAN-SUPERPOWERS-DATABASE.md](./HUMAN-SUPERPOWERS-DATABASE.md)) — mastery-tier skills connect to documented peak human performance
- **AI-Human Complementarity Map** ([AI-HUMAN-COMPLEMENTARITY-MAP.md](./AI-HUMAN-COMPLEMENTARITY-MAP.md)) — every skill maps to its AI augmentation potential

---

## Skill Identifier Schema

Every skill follows the same canonical ID format as the knowledge catalog:

```
{audience}.{plane}.{category}.{slug}
```

| Field | Value | Example |
|---|---|---|
| `audience` | `hum` | `hum` |
| `plane` | `skill` | `skill` |
| `category` | kebab-case category name | `cognitive` |
| `slug` | kebab-case skill name | `critical-thinking` |

**Full example:** `hum.skill.cognitive.critical-thinking`

---

## Skill Node Schema

Every skill in the HSD is a structured node:

```json
{
  "id": "hum.skill.cognitive.critical-thinking",
  "title": "Critical Thinking",
  "slug": "critical-thinking",
  "category": "cognitive",
  "description": "The ability to analyse information objectively, identify logical fallacies, evaluate evidence, and form well-reasoned judgements.",
  "knowledge_prerequisites": [
    "hum.know.basic.basic-logic-and-reasoning",
    "hum.know.basic.basic-critical-thinking"
  ],
  "skill_prerequisites": [],
  "proficiency_levels": [
    { "level": 1, "label": "Beginner", "description": "Identifies obvious logical fallacies; asks clarifying questions" },
    { "level": 2, "label": "Developing", "description": "Evaluates arguments; distinguishes fact from opinion" },
    { "level": 3, "label": "Competent", "description": "Analyses complex problems; constructs reasoned arguments" },
    { "level": 4, "label": "Proficient", "description": "Applies Socratic method; evaluates competing frameworks" },
    { "level": 5, "label": "Mastery", "description": "Operates at the frontier of epistemic rigour; teaches the skill" }
  ],
  "learning_pathway": [
    "Study formal logic and argument structure",
    "Practice Socratic questioning in daily conversations",
    "Analyse and deconstruct 100 real-world arguments",
    "Study cognitive biases and practice bias detection",
    "Teach critical thinking to others"
  ],
  "assessment_method": "Argument analysis exercises; logical fallacy identification tests; structured debate performance",
  "ai_augmentation": "AI can surface counterarguments, fact-check claims in real-time, and map argument structure — but the human judgement call remains human",
  "ai_replicable": false,
  "superpower_ceiling": "hum.superpower.cognitive.master-debater",
  "tacit_component": "high",
  "version": "1.0.0",
  "last_validated": "2026-09-21"
}
```

### Schema Field Reference

| Field | Type | Description |
|---|---|---|
| `id` | `string` | Canonical skill ID |
| `title` | `string` | Human-readable skill name |
| `slug` | `string` | kebab-case identifier |
| `category` | `enum` | Skill category (see taxonomy below) |
| `description` | `string` | One-sentence skill summary |
| `knowledge_prerequisites` | `string[]` | HKD domain IDs required before this skill can be developed |
| `skill_prerequisites` | `string[]` | Other HSD skill IDs that must be developed first |
| `proficiency_levels` | `object[]` | 5-level proficiency ladder with descriptors |
| `learning_pathway` | `string[]` | Ordered steps from beginner to mastery |
| `assessment_method` | `string` | How proficiency at each level is measured |
| `ai_augmentation` | `string` | How AI can augment (not replace) this skill |
| `ai_replicable` | `boolean` | Whether AI can fully replicate this skill |
| `superpower_ceiling` | `string\|null` | Linked Superpower node representing peak human performance |
| `tacit_component` | `enum` | `low` \| `medium` \| `high` — how much of this skill is tacit/embodied |
| `version` | `semver` | Schema version |
| `last_validated` | `ISO 8601` | Date last reviewed |

---

## Skill Taxonomy — 8 Categories

### 1. Cognitive Skills (`hum.skill.cognitive.*`)

Mental processes for thinking, reasoning, learning, and problem-solving.

| Skill ID | Title | Tacit Component | AI Replicable |
|---|---|---|---|
| `hum.skill.cognitive.critical-thinking` | Critical Thinking | High | No |
| `hum.skill.cognitive.problem-solving` | Problem Solving | High | Partial |
| `hum.skill.cognitive.creativity` | Creativity & Ideation | High | No |
| `hum.skill.cognitive.memory-and-recall` | Memory & Recall | Medium | Partial |
| `hum.skill.cognitive.focus-and-concentration` | Focus & Concentration | High | No |
| `hum.skill.cognitive.pattern-recognition` | Pattern Recognition | Medium | Partial |
| `hum.skill.cognitive.abstract-reasoning` | Abstract Reasoning | Medium | Partial |
| `hum.skill.cognitive.systems-thinking` | Systems Thinking | High | Partial |
| `hum.skill.cognitive.decision-making` | Decision Making Under Uncertainty | High | No |
| `hum.skill.cognitive.intuition` | Intuition & Gut Reasoning | High | No |
| `hum.skill.cognitive.mental-models` | Mental Model Building | High | Partial |
| `hum.skill.cognitive.research` | Research & Synthesis | Medium | Partial |

### 2. Communication Skills (`hum.skill.communication.*`)

The ability to convey, receive, and exchange meaning effectively across contexts.

| Skill ID | Title | Tacit Component | AI Replicable |
|---|---|---|---|
| `hum.skill.communication.public-speaking` | Public Speaking | High | No |
| `hum.skill.communication.writing` | Writing (Expressive & Technical) | High | Partial |
| `hum.skill.communication.active-listening` | Active Listening | High | No |
| `hum.skill.communication.storytelling` | Storytelling | High | No |
| `hum.skill.communication.persuasion` | Persuasion & Rhetoric | High | No |
| `hum.skill.communication.negotiation` | Negotiation | High | No |
| `hum.skill.communication.nonverbal` | Nonverbal Communication | High | No |
| `hum.skill.communication.cross-cultural` | Cross-Cultural Communication | High | No |
| `hum.skill.communication.facilitation` | Meeting & Group Facilitation | High | No |
| `hum.skill.communication.teaching` | Teaching & Explanation | High | No |

### 3. Technical Skills (`hum.skill.technical.*`)

Applied technical capabilities requiring knowledge + practice.

| Skill ID | Title | Tacit Component | AI Replicable |
|---|---|---|---|
| `hum.skill.technical.programming` | Programming & Software Development | Medium | Partial |
| `hum.skill.technical.engineering-design` | Engineering Design & Prototyping | High | Partial |
| `hum.skill.technical.data-analysis` | Data Analysis & Visualisation | Medium | Partial |
| `hum.skill.technical.scientific-method` | Scientific Method & Experimentation | High | Partial |
| `hum.skill.technical.systems-design` | Systems Design & Architecture | High | Partial |
| `hum.skill.technical.mathematics-applied` | Applied Mathematics | Medium | Partial |
| `hum.skill.technical.electronics` | Electronics & Hardware | High | No |
| `hum.skill.technical.fabrication` | Fabrication & Making | High | No |
| `hum.skill.technical.medical-procedures` | Medical & Clinical Procedures | High | No |
| `hum.skill.technical.navigation` | Navigation (Land, Sea, Air) | High | Partial |

### 4. Physical Skills (`hum.skill.physical.*`)

Bodily capabilities, motor skills, and physical performance.

| Skill ID | Title | Tacit Component | AI Replicable |
|---|---|---|---|
| `hum.skill.physical.athletics-speed` | Speed & Sprinting | High | No |
| `hum.skill.physical.athletics-endurance` | Endurance & Stamina | High | No |
| `hum.skill.physical.athletics-strength` | Strength & Power | High | No |
| `hum.skill.physical.athletics-agility` | Agility & Coordination | High | No |
| `hum.skill.physical.fine-motor` | Fine Motor Control & Precision | High | No |
| `hum.skill.physical.spatial-awareness` | Spatial Awareness & Balance | High | No |
| `hum.skill.physical.dance` | Dance & Movement Arts | High | No |
| `hum.skill.physical.craftsmanship` | Craftsmanship & Artisanal Making | High | No |
| `hum.skill.physical.surgical-precision` | Surgical & Micro-Manual Precision | High | No |
| `hum.skill.physical.martial-arts` | Martial Arts & Combat | High | No |
| `hum.skill.physical.wilderness-survival` | Wilderness Survival | High | Partial |

### 5. Social Skills (`hum.skill.social.*`)

Interpersonal and collective human capabilities.

| Skill ID | Title | Tacit Component | AI Replicable |
|---|---|---|---|
| `hum.skill.social.empathy` | Empathy & Emotional Attunement | High | No |
| `hum.skill.social.leadership` | Leadership & Inspiration | High | No |
| `hum.skill.social.collaboration` | Collaboration & Teamwork | High | No |
| `hum.skill.social.mentoring` | Mentoring & Coaching | High | No |
| `hum.skill.social.conflict-resolution` | Conflict Resolution & Mediation | High | No |
| `hum.skill.social.networking` | Relationship Building & Networking | High | No |
| `hum.skill.social.community-building` | Community Building & Organising | High | No |
| `hum.skill.social.parenting` | Parenting & Child Development | High | No |
| `hum.skill.social.cultural-intelligence` | Cultural Intelligence (CQ) | High | No |
| `hum.skill.social.political-acumen` | Political & Diplomatic Acumen | High | No |

### 6. Metacognitive Skills (`hum.skill.metacognitive.*`)

Skills about managing and improving one's own thinking and learning.

| Skill ID | Title | Tacit Component | AI Replicable |
|---|---|---|---|
| `hum.skill.metacognitive.self-awareness` | Self-Awareness & Introspection | High | No |
| `hum.skill.metacognitive.learning-how-to-learn` | Learning How to Learn | High | Partial |
| `hum.skill.metacognitive.reflection` | Reflective Practice | High | No |
| `hum.skill.metacognitive.goal-setting` | Goal Setting & Self-Direction | High | Partial |
| `hum.skill.metacognitive.emotional-regulation` | Emotional Regulation | High | No |
| `hum.skill.metacognitive.resilience` | Resilience & Adversity Navigation | High | No |
| `hum.skill.metacognitive.mindfulness` | Mindfulness & Present-Moment Awareness | High | No |
| `hum.skill.metacognitive.habit-formation` | Habit Formation & Behaviour Change | High | Partial |

### 7. Creative Skills (`hum.skill.creative.*`)

The capacity to generate original works, ideas, and expressions.

| Skill ID | Title | Tacit Component | AI Replicable |
|---|---|---|---|
| `hum.skill.creative.visual-art` | Visual Art & Drawing | High | Partial |
| `hum.skill.creative.musical-performance` | Musical Performance & Improvisation | High | No |
| `hum.skill.creative.composition` | Musical Composition | High | Partial |
| `hum.skill.creative.creative-writing` | Creative & Literary Writing | High | Partial |
| `hum.skill.creative.invention` | Invention & Innovation | High | No |
| `hum.skill.creative.design-thinking` | Design Thinking & Human-Centred Design | High | Partial |
| `hum.skill.creative.architecture-design` | Architectural & Spatial Design | High | Partial |
| `hum.skill.creative.filmmaking` | Filmmaking & Visual Storytelling | High | Partial |
| `hum.skill.creative.culinary-art` | Culinary Arts & Flavour Craft | High | No |
| `hum.skill.creative.comedy-and-play` | Comedy, Play & Humour | High | No |

### 8. Survival & Practical Skills (`hum.skill.survival.*`)

Fundamental human capabilities for living, making, and thriving in the physical world.

| Skill ID | Title | Tacit Component | AI Replicable |
|---|---|---|---|
| `hum.skill.survival.first-aid` | First Aid & Emergency Response | High | Partial |
| `hum.skill.survival.food-growing` | Food Growing & Agriculture | High | No |
| `hum.skill.survival.cooking` | Cooking & Food Preparation | High | Partial |
| `hum.skill.survival.shelter-building` | Shelter Building & Construction | High | No |
| `hum.skill.survival.tool-use` | Tool Use & Repair | High | No |
| `hum.skill.survival.fire-making` | Fire Making & Management | High | No |
| `hum.skill.survival.water-sourcing` | Water Sourcing & Purification | High | Partial |
| `hum.skill.survival.wayfinding` | Wayfinding & Natural Navigation | High | Partial |
| `hum.skill.survival.animal-husbandry` | Animal Husbandry & Care | High | No |
| `hum.skill.survival.financial-management` | Personal Financial Management | Medium | Partial |

---

## Proficiency Level Framework

All skills use a universal 5-level proficiency ladder:

| Level | Label | General Description |
|---|---|---|
| 1 | **Beginner** | Aware of the skill; can perform basic elements with guidance |
| 2 | **Developing** | Can perform independently in simple, familiar contexts |
| 3 | **Competent** | Reliable performance across standard contexts; adapts to variation |
| 4 | **Proficient** | High performance in complex contexts; can teach others |
| 5 | **Mastery** | Operates at the frontier; recognised expert; pushes the skill forward |

Level 5 Mastery connects to the **Human Superpowers Database** where documented peak performers represent the outer edge of what is humanly possible.

---

## Learning Pathway Engine

The Learning Pathway Engine generates structured, personalised paths to skill mastery for any GAIAN twin. Given a target skill and current proficiency level, it returns:

1. **Knowledge prerequisites** to load first (from HKD)
2. **Skill prerequisites** to develop first (from HSD)
3. **Ordered learning steps** from current level to target level
4. **Practice resources** linked to authoritative sources
5. **Assessment checkpoints** with measurable outcomes
6. **AI augmentation tools** available at each stage
7. **Estimated time investment** based on documented learning research

The engine is powered by the Graphiti knowledge graph traversal layer, resolving all dependency chains before generating the pathway.

---

## GAIAN Skill Profile Integration

Every GAIAN twin maintains a personal Skill Profile stored in the Mi-Memory Framework ([#644](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/644)):

```json
{
  "gaian_id": "did:gaia:twin:abc123",
  "skill_profile": [
    {
      "skill_id": "hum.skill.cognitive.critical-thinking",
      "proficiency_level": 3,
      "last_assessed": "2026-09-01",
      "learning_pathway_active": true,
      "next_milestone": "Complete 100 argument analyses"
    }
  ]
}
```

Skill profiles are private by default. GAIANs may choose to share specific skills publicly or with specific communities.

---

## Acceptance Criteria (from Issue #626)

- [ ] Skills taxonomy complete with proficiency levels for all 8 categories (81 skills)
- [ ] Learning pathway engine generates valid paths for any skill query
- [ ] Skill assessment framework documented for all skills
- [ ] GAIAN skill profile stores and retrieves personal skill levels via Mi-Memory
- [ ] AI-human complementarity map covers all 81 skills
- [ ] All skills linked to relevant HKD knowledge prerequisites
- [ ] Mastery-level skills linked to Human Superpowers Database ceiling
- [ ] `validate_catalog.py` extended to validate skills catalog

---

## Related Issues

| Issue | Title |
|---|---|
| [#620](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/620) | [EPIC] Knowledge & Intelligence Database System |
| [#625](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/625) | Human Knowledge Database |
| [#627](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/627) | Unified Knowledge API |
| [#644](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/644) | Mi-Memory Framework |
| [#684](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/684) | GAIAN 2.0 Sentient Architecture |
