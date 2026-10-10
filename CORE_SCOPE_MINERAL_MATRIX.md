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

The entries below are literature baselines, not results from the proposed apparatus. A reference value is not a measurement of the particular specimen. Specimen-level PASS requires an identified sample, provenance, method, calibration, raw data, uncertainty, and reproducible result. The word PASS elsewhere in this document is restricted to the explicitly named scope or verification gate; it must not be read as evidence that a mineral sample or Option B mechanism has passed an experiment.

| # | Material | Literature-supported baseline | Experimental role | Evidence state |
|---|---|---|---|---|
| 1 | Ametrine | Quartz variety with purple/yellow-orange color zones [1, 11]; quartz reference RI values are approximately ω=1.544 and ε=1.553 [1] | Measure polarization/phase response; do not assume the sample acts as a beam splitter | LITERATURE BASELINE; specimen unmeasured |
| 2 | Amphibole Quartz | Quartz host with amphibole inclusions; inclusion identity, distribution, and optical effect must be verified per specimen [1] | Characterize inclusions and measure scattering/transmission | LITERATURE BASELINE; specimen unmeasured |
| 3 | Anandalite | “Iris quartz” reports describe interference associated with thin parallel twin planes and/or depositional layers; name and specimen identification require care [2] | Verify identity and measure ordinary optical interference | REPORTED LITERATURE EFFECT; specimen unverified |
| 4 | Ajoite-bearing Quartz | Ajoite inclusions have been documented in quartz; inclusion distribution and optical response vary by specimen [3] | Verify mineral identification; measure transmission and absorption spectra | LITERATURE BASELINE; specimen unmeasured |
| 5 | Ammolite | Iridescence in ammolite is associated with light interference from stacked aragonite platelets; fossilization and specimen structure vary [4] | Characterize layer structure and angle/wavelength response | ESTABLISHED GENERAL EFFECT; specimen unmeasured |
| Core | Obsidian | Naturally occurring volcanic glass, commonly rhyolitic; composition and specimen condition affect optical behavior [5] | Measure each plate's composition, surface, transmission, reflection, and scattering | GENERAL GEOLOGICAL BASELINE; specimen unmeasured |
| 6 | Aragonite | Orthorhombic CaCO₃; birefringent, with reference optical constants listed in the mineralogical literature [6] | Measure orientation-dependent polarization/phase response | LITERATURE BASELINE; specimen unmeasured |
| 7 | Brookite | Orthorhombic TiO₂ polymorph with published optical constants; sample quality and orientation affect response [7] | Measure refractive/anisotropic response before assigning an optical role | LITERATURE BASELINE; specimen unmeasured |
| 8 | Baryte | Orthorhombic BaSO₄; mineral references report high specific gravity and optical anisotropy [8] | Measure sample mass/dimensions and optical/mechanical properties | LITERATURE BASELINE; specimen unmeasured |
| 9 | Black Kyanite | Kyanite has directional hardness and biaxial optical properties; pleochroism and measured properties depend on orientation/thickness [9] | Identify sample and measure orientation-dependent response | LITERATURE BASELINE; specimen unmeasured |
| 10 | Astrophyllite | Mineralogical references describe biaxial optical behavior and strong pleochroism; exact chemistry and optical response require specimen characterization [10] | Verify identity/composition and measure polarization-dependent absorption/scattering | LITERATURE BASELINE; specimen unmeasured |

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

- Ten-mineral matrix as an experimental design: DESIGN PASS (scope completeness only; not experimental validation)
- Individual physical properties: LITERATURE BASELINES CITED; no specimen-level measurements are supplied by this document
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

## References — General Material Baselines

These references support general mineralogical or gemological descriptions only. They do not certify the identity, purity, dimensions, optical constants, or behavior of any specimen selected for this apparatus.

1. Mineralogical Society of America / Mineral Data Publishing, *Handbook of Mineralogy: Quartz* (crystal data and optical properties): https://www.handbookofmineralogy.org/pdfs/quartz.pdf
2. Alfredo Petrov and Yuko Tanaka, “Iris Quartz” (reported thin-structure interference; terminology/specimen caveats): https://www.mindat.org/article.php/1335/Iris%2BQuartz
3. Mindat, “Ajoite” and documented ajoite-in-quartz specimens (mineral identification and inclusion examples): https://www.mindat.org/a/best_ajoite and https://www.mindat.org/photo-1741982.html
4. Mychaluk, Levinson, and Hall, “Ammolite: Iridescent Fossilized Ammonite from Southern Alberta, Canada,” *Gems & Gemology*, GIA (2001): https://www.gia.edu/gems-gemology/wn13-ammolite-organic-jewel-cole
5. U.S. Geological Survey, “Volcano Watch — Obsidian, a scarce commodity in Hawaiʻi” (obsidian as volcanic glass and compositional context): https://www.usgs.gov/news/volcano-watch-obsidian-a-scarce-commodity-hawaii
6. Mineralogical Society of America / Mineral Data Publishing, *Handbook of Mineralogy: Aragonite*: https://www.handbookofmineralogy.org/pdfs/aragonite.pdf
7. Mineralogical Society of America / Mineral Data Publishing, *Handbook of Mineralogy: Brookite*: https://www.handbookofmineralogy.org/pdfs/brookite.pdf
8. Mineralogical Society of America / Mineral Data Publishing, *Handbook of Mineralogy: Baryte*: https://www.handbookofmineralogy.org/pdfs/baryte.pdf
9. Mineralogical Society of America / Mineral Data Publishing, *Handbook of Mineralogy: Kyanite*: https://www.handbookofmineralogy.org/pdfs/kyanite.pdf
10. Mineralogical Society of America / Mineral Data Publishing, *Handbook of Mineralogy: Astrophyllite*: https://www.handbookofmineralogy.org/pdfs/astrophyllite.pdf
11. Mindat, “Ametrine” (variety of quartz and color zoning): https://www.mindat.org/show.php?id=7606

## Specimen-Level Evidence Record (Required Before Experimental PASS)

For every specimen, record a unique specimen ID; supplier/locality and provenance; mineral identification method and confidence; dimensions, mass, cut/orientation, surface finish, inclusions and treatment; instrument make/model and calibration record; wavelength, polarization, geometry, temperature and environmental conditions; raw files and hashes; analysis code/version; uncertainty; controls; preregistration reference; and independent replication status. If an item is not available, mark it UNKNOWN or HOLD rather than inferring it from the literature baseline.
