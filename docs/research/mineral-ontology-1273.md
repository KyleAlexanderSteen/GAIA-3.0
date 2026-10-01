# Mineral ontology, listed

Partial for #1273 and #1171. No Mindat harvest. No graph database. No runtime ingest.

Checked 2026-10-01. The crystal catalog remains the pairing table. This file is the scientific schema those rows do not have.

## What a mineral row has to carry

IMA-CNMNC is the nomenclature authority. A species is not a label we invent. The commission's reports cover the definition of a mineral, type specimens, discreditation, end-member formula, polymorphs and polysomes, symbols, prefixes and suffixes, and the dominant-constituent rule. Relevant reports include Mills et al. 2009 on group hierarchies, Warr 2021 on approved symbols, and the 2023 polymorph and polysome guidelines. A GAIA row that names a species without an IMA symbol or an explicit "not IMA" flag is NeedVerify.

Nickel–Strunz, ninth edition revised by Nickel, published 2001, is the classification the IMA/CNMNC database supports. The code is `NN.XY.##x`: class, division, family, group. Ten classes:

1. Elements
2. Sulfides and sulfosalts
3. Halides
4. Oxides and hydroxides
5. Carbonates and nitrates
6. Borates
7. Sulfates, chromates, molybdates, tungstates
8. Phosphates, arsenates, vanadates
9. Silicates
10. Organic compounds

Mindat and Webmineral arrange species on this tree. GAIA does not copy either database here. The class number is the field a later harvest would fill.

Crystal system is one of seven: triclinic, monoclinic, orthorhombic, tetragonal, trigonal, hexagonal, cubic. Formula is the end-member, not a bulk analysis, unless the row says which.

## Standards mapping

W3C Recommendations unless noted. No crate implements them in this cut.

| Standard | Date | Use |
| --- | --- | --- |
| RDF 1.1 Concepts | 25 Feb 2014 | A row is triples. IRI, blank node, or literal. |
| RDF Schema 1.1 | 25 Feb 2014 | Class and property names. Not a reasoner. |
| SKOS | 18 Aug 2009 | Nickel–Strunz class/division/family/group as broader/narrower. `skos:prefLabel` is the IMA name. `skos:altLabel` is a synonym. `skos:notation` is the Strunz code. |
| OWL 2 | 11 Dec 2012 | Not used. A later issue can add class restrictions. |
| PROV-O | 30 Apr 2013 | `prov:wasDerivedFrom` is the source. `prov:generatedAtTime` is the observed date. |
| SPARQL 1.1 | 21 Mar 2013 | Named only. No store. |

RDF 1.2 was still a candidate and draft family on 24 Sep 2026. It is not the baseline.

## Required fields

| Field | Shape | Required |
| --- | --- | --- |
| mineral_id | IRI or stable local id | yes |
| pref_label | skos:prefLabel | yes |
| ima_symbol | Warr 2021 symbol, or `not-ima` | yes |
| strunz | `NN.XY.##x` or empty with reason | no |
| formula | end-member literal | no |
| crystal_system | one of the seven | no |
| locality | literal plus source | no |
| property | name, value, unit | no |
| source | prov:wasDerivedFrom | yes |
| license | literal | yes |
| observed | xsd:date | yes |

Empty source, license, observed date, or IMA flag is NeedVerify.

## Witness, not a catalog

```csv
mineral_id,pref_label,ima_symbol,strunz,formula,crystal_system,locality,property,source,license,observed
example:quartz,quartz,not-ima,,SiO2,hexagonal,,hardness=7,schema-witness,not-a-license,2026-10-01
```

This row is a schema witness. It is not a Mindat record and it is not an IMA import. Do not cite it as either.
