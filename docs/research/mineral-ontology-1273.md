# Mineral ontology, listed

Partial for #1273 and #1171. No Mindat harvest. No graph database. No runtime ingest.

Checked 2026-10-01. The crystal catalog remains the pairing table. This file is the scientific schema those rows do not have.

## Standards

These are W3C Recommendations unless noted. GAIA does not implement them in a crate here.

| Standard | Date | Use for a mineral row |
| --- | --- | --- |
| RDF 1.1 Concepts | 25 Feb 2014 | A row is a set of triples: subject, predicate, object. IRI, blank node, or literal. |
| RDF Schema 1.1 | 25 Feb 2014 | Class and property names. Not a reasoner. |
| SKOS Reference | 18 Aug 2009 | Taxonomy: broader, narrower, related, prefLabel. Mindat-style trees map here. |
| OWL 2 | 11 Dec 2012 | Only if a later issue needs class restrictions. Not used in this cut. |
| PROV-O | 30 Apr 2013 | Source, activity, agent. A row without a source is NeedVerify. |
| SPARQL 1.1 | 21 Mar 2013 | Query language, if a store exists. No store in this cut. |

RDF 1.2 was a Candidate Standard and draft family as of 24 Sep 2026. It is not the baseline. Do not bind the schema to a draft.

## Row

A mineral record needs these fields. Empty source, license, or observed time is NeedVerify, not a fact.

| Field | Predicate shape | Required |
| --- | --- | --- |
| mineral_id | IRI or stable local id | yes |
| pref_label | skos:prefLabel | yes |
| formula | literal | no |
| crystal_system | literal | no |
| locality | literal plus source | no |
| property | name, value, unit | no |
| source | prov:wasDerivedFrom | yes |
| license | literal | yes |
| observed | xsd:date | yes |

## Refusal

This cut does not fetch Mindat, Webmineral, or a geochemistry feed. A listed example below is a schema witness, not a catalog import.

```csv
mineral_id,pref_label,formula,crystal_system,locality,property,source,license,observed
example:quartz,quartz,SiO2,hexagonal,,hardness=7,schema-witness,not-a-license,2026-10-01
```

The example row has no locality and a fake license. It is not a Mindat record. Do not cite it as one.
