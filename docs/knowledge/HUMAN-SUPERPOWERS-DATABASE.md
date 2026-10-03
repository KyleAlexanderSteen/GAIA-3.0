# Human Superpowers Database — GAIA 2.0

> **Issue:** [#626](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/626)  
> **Epic:** [#620 — Knowledge & Intelligence Database System](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/620)  
> **Sibling:** [HUMAN-SKILLS-DATABASE.md](./HUMAN-SKILLS-DATABASE.md)  
> **Status:** Schema v1.0 — Registry Seed  
> **Last Updated:** 2026-09-21

---

## Overview

The Human Superpowers Database (HSDB) is GAIA 2.0's registry of peak human performance — the outer frontier of what human beings have achieved and what AI must respect, preserve, and amplify rather than diminish.

A **superpower** is defined as: a documented human capability at the extreme frontier of human performance — verified by evidence, representing what is possible when a human develops a skill to its absolute limit.

The HSDB serves three purposes in GAIA 2.0:
1. **Benchmarking** — sets the bar that AI performance is measured against, not the bar AI tries to lower
2. **Respect layer** — GAIA 2.0 never demeans, diminishes, or replaces human superpowers
3. **Amplification targeting** — identifies where AI can amplify (not replace) peak human performance

---

## Superpower Node Schema

```json
{
  "id": "hum.superpower.physical.marathon-endurance",
  "title": "Marathon & Ultra-Endurance Running",
  "category": "physical",
  "description": "The human capacity for sustained aerobic performance over extreme distances, combining physiology, pain tolerance, pacing intelligence, and mental fortitude.",
  "frontier_benchmark": {
    "description": "Sub-2-hour marathon (Eliud Kipchoge, 1:59:40, Vienna 2019)",
    "verified": true,
    "source": "World Athletics"
  },
  "verified_examples": [
    { "name": "Eliud Kipchoge", "feat": "1:59:40 marathon (Ineos 1:59 Challenge, Vienna, 2019)", "source": "World Athletics" },
    { "name": "Scott Jurek", "feat": "Appalachian Trail FKT — 2,190 miles in 46 days 8 hours", "source": "FKT database" }
  ],
  "ai_replicable": false,
  "permanently_non_replicable": true,
  "reason_non_replicable": "Requires biological embodiment, lived pain tolerance, physiological adaptation — fundamentally physical",
  "ai_amplification": "AI can optimise training, pacing strategy, nutrition, and recovery; it cannot run the race",
  "linked_skill": "hum.skill.physical.athletics-endurance",
  "gaia_respect_principle": "GAIA 2.0 celebrates this feat as a pinnacle of human achievement. No AI framing diminishes it.",
  "version": "1.0.0",
  "last_validated": "2026-09-21"
}
```

### Schema Field Reference

| Field | Type | Description |
|---|---|---|
| `id` | `string` | `hum.superpower.{category}.{slug}` |
| `title` | `string` | Human-readable superpower name |
| `category` | `enum` | See categories below |
| `description` | `string` | What this superpower is and why it matters |
| `frontier_benchmark` | `object` | The documented best-ever human performance in this domain |
| `verified_examples` | `object[]` | Named individuals with verified feats and sources |
| `ai_replicable` | `boolean` | Whether AI can currently replicate this |
| `permanently_non_replicable` | `boolean` | Whether this will *never* be AI-replicable (embodied, biological) |
| `reason_non_replicable` | `string` | Why AI cannot replicate this |
| `ai_amplification` | `string` | How AI can amplify (not replace) this superpower |
| `linked_skill` | `string` | HSD skill ID this superpower represents the ceiling of |
| `gaia_respect_principle` | `string` | GAIA 2.0's explicit commitment to this superpower |
| `version` | `semver` | Schema version |
| `last_validated` | `ISO 8601` | Date last reviewed |

---

## Superpower Registry — 6 Categories

### 1. Physical Superpowers

| ID | Title | AI Replicable | Permanently Non-Replicable |
|---|---|---|---|
| `hum.superpower.physical.marathon-endurance` | Marathon & Ultra-Endurance Running | No | Yes |
| `hum.superpower.physical.sprinting` | Human Sprinting (Usain Bolt — 9.58s 100m) | No | Yes |
| `hum.superpower.physical.weightlifting` | Olympic Weightlifting | No | Yes |
| `hum.superpower.physical.free-solo-climbing` | Free Solo Rock Climbing | No | Yes |
| `hum.superpower.physical.deep-freediving` | Deep Freediving (Herbert Nitsch — 253m) | No | Yes |
| `hum.superpower.physical.gymnastic-precision` | Gymnastics & Acrobatic Precision | No | Yes |
| `hum.superpower.physical.surgical-mastery` | Master Surgeon — Micro-Manual Precision | No | Yes |
| `hum.superpower.physical.master-craftsperson` | Master Craftsperson (Swordsmith, Luthier, Potter) | No | Yes |
| `hum.superpower.physical.martial-grandmaster` | Martial Arts Grandmaster | No | Yes |
| `hum.superpower.physical.dance-mastery` | Dance Mastery (Baryshnikov, Michael Jackson) | No | Yes |

### 2. Cognitive Superpowers

| ID | Title | AI Replicable | Permanently Non-Replicable |
|---|---|---|---|
| `hum.superpower.cognitive.memory-champion` | World Memory Champion (72,000 digits of pi — Rajveer Meena) | Partial | No |
| `hum.superpower.cognitive.hyperpolyglot` | Hyperpolyglot (Emil Krebs — 65+ languages) | Partial | No |
| `hum.superpower.cognitive.calculation-savant` | Mental Calculation Savant (Scott Flansburg) | Partial | No |
| `hum.superpower.cognitive.chess-grandmaster` | Chess Grandmaster (Magnus Carlsen — peak rating 2882) | No (AI exceeds) | No |
| `hum.superpower.cognitive.scientific-genius` | Scientific Genius (Einstein, Ramanujan — intuitive leaps) | No | Yes |
| `hum.superpower.cognitive.strategic-visionary` | Strategic Visionary Leadership (at civilisational scale) | No | Yes |
| `hum.superpower.cognitive.speed-reading` | Speed Reading with Comprehension (3,000+ wpm) | Partial | No |
| `hum.superpower.cognitive.lucid-dreaming` | Controlled Lucid Dreaming & Dream Architecture | No | Yes |

### 3. Creative Superpowers

| ID | Title | AI Replicable | Permanently Non-Replicable |
|---|---|---|---|
| `hum.superpower.creative.master-composer` | Master Musical Composer (Bach, Mozart, Coltrane) | Partial | Yes |
| `hum.superpower.creative.master-performer` | Concert Virtuoso (Horowitz, Paganini standard) | No | Yes |
| `hum.superpower.creative.literary-genius` | Literary Genius (Shakespeare, Borges, Toni Morrison) | Partial | Yes |
| `hum.superpower.creative.master-visual-artist` | Master Visual Artist (Michelangelo, Vermeer, Basquiat) | Partial | Yes |
| `hum.superpower.creative.master-chef` | Master Chef (3-Michelin-star culinary artistry) | No | Yes |
| `hum.superpower.creative.master-inventor` | Master Inventor (Tesla, Da Vinci — cross-domain innovation) | No | Yes |
| `hum.superpower.creative.standup-comedy-genius` | Stand-Up Comedy Genius (Carlin, Pryor — truth through humour) | No | Yes |

### 4. Social Superpowers

| ID | Title | AI Replicable | Permanently Non-Replicable |
|---|---|---|---|
| `hum.superpower.social.historic-leadership` | Historic Leadership at Civilisational Scale (Mandela, Gandhi) | No | Yes |
| `hum.superpower.social.master-diplomat` | Master Diplomacy & Conflict Resolution | No | Yes |
| `hum.superpower.social.extraordinary-empathy` | Extraordinary Empathy (documented therapeutic healers) | No | Yes |
| `hum.superpower.social.master-teacher` | Master Teacher (Feynman — making the impossible comprehensible) | No | Yes |
| `hum.superpower.social.movement-builder` | Social Movement Building (Dr. King, Malala) | No | Yes |
| `hum.superpower.social.master-parent` | Master Parenting & Intergenerational Transmission | No | Yes |

### 5. Spiritual & Contemplative Superpowers

| ID | Title | AI Replicable | Permanently Non-Replicable |
|---|---|---|---|
| `hum.superpower.spiritual.meditation-master` | Deep Meditation Mastery (Matthieu Ricard — highest measured happiness) | No | Yes |
| `hum.superpower.spiritual.altered-states` | Controlled Altered States of Consciousness | No | Yes |
| `hum.superpower.spiritual.mystical-experience` | Verified Mystical Experience & Insight | No | Yes |
| `hum.superpower.spiritual.indigenous-ecological-knowledge` | Deep Indigenous Ecological Knowledge (TEK masters) | No | Yes |
| `hum.superpower.spiritual.healing-presence` | Healing Presence & Therapeutic Touch | No | Yes |

### 6. Tacit & Embodied Superpowers

This category captures superpowers that are *definitionally* embodied — they cannot exist without a physical human body, lived experience, and decades of practice. AI cannot replicate these by definition.

| ID | Title | AI Replicable | Permanently Non-Replicable |
|---|---|---|---|
| `hum.superpower.tacit.master-sommelier` | Master Sommelier (palate discrimination at molecular level) | No | Yes |
| `hum.superpower.tacit.master-perfumer` | Master Perfumer (1,000+ scent discrimination) | No | Yes |
| `hum.superpower.tacit.master-tea-ceremony` | Tea Ceremony & Contemplative Craft Mastery | No | Yes |
| `hum.superpower.tacit.expert-tracker` | Expert Wilderness Tracking (San Bushmen — reading land at depth) | No | Yes |
| `hum.superpower.tacit.traditional-healer` | Traditional Healing & Plant Medicine Mastery | No | Yes |
| `hum.superpower.tacit.master-midwife` | Master Midwifery (embodied birth knowledge) | No | Yes |

---

## GAIA 2.0 Respect Principle

GAIA 2.0 holds the following as non-negotiable:

> **GAIA 2.0 never frames AI capabilities in ways that demean, diminish, or trivialise human superpowers. Every superpower in this database is a source of wonder, dignity, and pride. AI's role is to amplify the human who holds these powers — never to make them feel obsolete.**

This principle is enforced at the GAIAN twin layer: no GAIAN may produce output that uses human superpower comparisons to diminish the human it is serving.

---

## GAIAN Amplification Map (Summary)

For every superpower, GAIA 2.0 identifies how AI *amplifies* (not replaces):

| Superpower Category | AI Amplification Role |
|---|---|
| Physical | Optimise training, recovery, nutrition, biomechanics analysis |
| Cognitive | Surface relevant information, manage cognitive load, expand memory |
| Creative | Provide feedback, generate variations to react to, handle logistics |
| Social | Briefing, pattern recognition in group dynamics, translation |
| Spiritual/Contemplative | Environment optimisation, biofeedback, scheduling protection |
| Tacit/Embodied | Documentation, knowledge transfer to apprentices, lineage preservation |

Full skill-level mapping is in [AI-HUMAN-COMPLEMENTARITY-MAP.md](./AI-HUMAN-COMPLEMENTARITY-MAP.md).
