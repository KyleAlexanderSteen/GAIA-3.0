# CORE_SCOPE_MINERAL_MATRIX

## Status
Option B / Spirit Physics experimental scope. State: DESIGN / HOLD.

The ten-mineral matrix is an experimental apparatus. Mineral properties are treated as physical baselines; proposed non-local, informational, field, or 12-layer mechanisms remain hypotheses. UNKNOWN is preserved.

## Architecture

```
COHERENT LASER
  -> Ametrine
  -> Amphibole Quartz
  -> Anandalite / Iris Quartz
  -> Ajoite-bearing Quartz
  -> Ammolite
  -> Dual-Plated Obsidian
  -> CMOS / interferometric sensor
  -> calibration -> physical observables -> null models -> residual analysis

Boundary matrix:
Aragonite | Brookite | Baryte | Black Kyanite | Astrophyllite
```

## Physical Baseline

| # | Material | Physical baseline | Experimental role | State |
|---|---|---|---|---|
| 1 | Ametrine | Quartz; trigonal; RI about 1.544-1.553; birefringence about 0.009 | Birefringent optical element | PASS/HOLD |
| 2 | Amphibole Quartz | Quartz with amphibole inclusions; heterogeneous scattering/absorption/multipath effects | Heterogeneous optical element | HOLD |
| 3 | Anandalite | Iris quartz; thin parallel twin/depositional structures produce interference | Thin-structure interference element | PASS/HOLD |
| 4 | Ajoite-bearing Quartz | Copper-bearing inclusions; specimen-dependent absorption/scattering | Spectral characterization element | HOLD |
| 5 | Ammolite | Layered aragonite platelet structure; angle/wavelength-dependent interference | Multilayer optical element | PASS |
| Core | Obsidian | Volcanic glass; transmission/reflection/scattering are specimen-dependent | Optical interaction zone | HOLD |
| 6 | Aragonite | Orthorhombic CaCO3; very strong birefringence, about 0.155 | Strong anisotropy reference | PASS |
| 7 | Brookite | Orthorhombic TiO2; high RI and substantial birefringence | High-index anisotropic element | PASS |
| 8 | Baryte | Orthorhombic BaSO4; SG about 4.3-4.5; optical anisotropy | Dense material/mechanical control | PASS |
| 9 | Black Kyanite | Strong directional mechanical/optical anisotropy | Orientation-dependent control | PASS/HOLD |
| 10 | Astrophyllite | Complex Fe/Mn/Ti silicate; strong pleochroism/optical anisotropy | Absorption/scattering control | PASS/HOLD |

### Guardrails

- Ametrine is not automatically a binary orthogonal beam splitter; output depends on cut, orientation, polarization and geometry.
- Amphibole inclusions are not automatically coherent fiber-optic waveguides; waveguiding must be measured.
- Anandalite does not establish a quantum phase detector; first measure ordinary optical phase/interference.
- Ajoite is not assumed to be a narrowband IR filter; measure transmission and absorption spectra.
- Ammolite is an interference structure, not a guaranteed diffraction gate.
- Brookite is a high-index anisotropic optical material; unusual wave behavior must be measured.
- Baryte's density is not gravitational or magnetic shielding.
- Kyanite's directional hardness does not establish electromagnetic noise routing.
- Astrophyllite is not assumed to absorb off-axis photons until specimen-level behavior is measured.

## Obsidian Measurement Zone

Treat the two plates initially as a controlled double-pass optical arrangement, not automatically as a resonant cavity.

Record plate thickness/separation, wavelength, incidence angle, polarization, reflectivity, roughness, refractive index, transmission, composition, inclusions and temperature.

A cavity claim requires independently justified reflectivity, phase condition, geometry and Q.

## Domain Translation Matrix

| Component | Physical observable | Option B interpretation | State |
|---|---|---|---|
| Ametrine | birefringence, phase delay, polarization | dual-state representation | HOLD |
| Amphibole quartz | scattering, multipath, transmission | multichannel relational paths | HOLD |
| Anandalite | interference/fringe structure | phase-sensitive representation | HOLD |
| Ajoite quartz | spectral attenuation | spectral filtering | HOLD |
| Ammolite | angle/wavelength interference | structured optical encoding | HOLD |
| Obsidian | phase/amplitude/scattering | heterogeneous mixing zone | HOLD |
| Aragonite | birefringence | boundary anisotropy | HOLD |
| Brookite | high-index refraction/birefringence | boundary modulation | HOLD |
| Baryte | density/optical/mechanical response | environmental control variable | HOLD |
| Kyanite | directional optical/electrical response | anisotropic control variable | HOLD |
| Astrophyllite | pleochroism/absorption/scattering | off-axis control | HOLD |

## Experimental Sequence

### Phase 0 — Material characterization
Record provenance, dimensions, mass, orientation, surface condition, transmission, reflection, polarization response, temperature and relevant environmental variables.

### Phase 1 — Obsidian baseline
Run the dual-plated obsidian without the mineral matrix. Record raw CMOS data and environmental telemetry. Characterize detector, laser and environmental noise rather than assuming sensor noise is purely thermal.

### Phase 2 — Grooming chain
Add minerals 1-5 sequentially. Preserve raw data after every addition. Measure intensity, spatial/temporal spectra, fringe visibility and polarization where available.

### Phase 3 — Boundary matrix
Compare ring OFF, ring ON, sham ring, randomized mineral positions and individual-mineral controls.

### Phase 4 — Independent QRNG
Acquire the QRNG stream independently. Do not define success as a QRNG entropy decrease. Test preregistered QRNG/optical relationships while controlling timing, power, RF/EM interference, vibration, temperature, laser state and software artifacts.

## Null Models

- H0-A: ordinary optical physics + measured material properties
- H0-B: environmental perturbation
- H0-C: detector/laser noise
- H0-D: mechanical/thermal/acoustic coupling
- H0-E: ordinary statistical dependence
- H0-F: software/timing/data-pipeline artifact
- HB: H0 plus a specifically defined Option B coupling

HB cannot be evaluated until the added coupling has an operational definition and discriminating prediction.

## Information Analysis

Candidate observables include intensity, optical phase, fringe displacement/visibility, polarization, spatial/temporal Fourier spectra, autocorrelation, cross-correlation, mutual/conditional mutual information, entropy estimates and compression-based complexity estimates.

True Kolmogorov complexity is uncomputable; practical implementations must declare an estimator.

A lower Shannon entropy in a QRNG stream is not by itself evidence of non-local information.

## Verification Gate

A candidate Option B result requires:

1. preregistered observable and effect range
2. blinded analysis where feasible
3. environmental and instrumental controls
4. sham/material/permutation controls
5. temporal-order test
6. surrogate-data test
7. multiple-comparison control
8. replication
9. competing-model comparison
10. quantitative prediction discriminating Option B from viable nulls

Classification:
- PASS = discriminating prediction survives controls and replication
- HOLD = anomaly exists but alternatives remain
- FAIL = prediction absent or explained by a null
- UNKNOWN = evidence insufficient

## 12-Layer Boundary

The 12-layer/3x4 model is downstream:

RAW DATA -> CALIBRATION -> PHYSICAL OBSERVABLES -> ENVIRONMENTAL MODEL -> NULL-MODEL COMPARISON -> RESIDUAL -> BLINDED STATISTICS -> REPLICATION -> 12-LAYER REPRESENTATION -> OPTION B INTERPRETATION

The representation must not manufacture evidence for the hypothesis it evaluates.

## Research Questions

1. Does the ten-mineral matrix produce reproducible optical changes beyond ordinary material models?
2. Can each mineral's contribution be independently characterized?
3. Does the boundary arrangement alter measurable environmental/optical stability?
4. Does that effect survive sham and permutation controls?
5. Does any QRNG/optical relationship survive temporal, environmental, instrumental and surrogate controls?
6. Does a quantitative observation distinguish Option B from ordinary optical/information models?
7. Is any observed effect substrate-independent or material-specific?
8. Does the 12-layer model add predictive value beyond ordinary representations?

## Current Classification

- Ten-mineral matrix as an experimental design: PASS
- Individual physical properties: PASS where independently characterized
- Proposed optical roles: HOLD pending specimen-level measurement
- Environmental damping: HOLD
- Field focusing: UNKNOWN
- Gravitational shielding: REJECTED as an established mechanism
- Magnetic shielding by baryte: REJECTED as an established mechanism
- EM routing by kyanite: UNKNOWN pending measurement
- QRNG-optical coupling: UNKNOWN
- Spirit-specific observable: HOLD
- 12-layer physical necessity: UNKNOWN
- Option B mechanism: UNKNOWN
- Overall program: HOLD / research-ready

## Evidence Rule

Observation -> Interpretation -> Claim -> Evidence -> Verification -> Classification.

Preserve raw data, provenance, controls, residuals, alternative explanations and UNKNOWN states.

The apparatus may test the hypothesis. It must never assume the hypothesis.
