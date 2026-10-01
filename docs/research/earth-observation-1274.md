# Earth observation and twin maturity, listed

Partial for #1274 and #1275. No ingest. Does not close #639, #654, #729, or #730.

Checked 2026-10-01. GAIA Earth remains a fixture ensemble. These rows are citations, not feeds.

## Dataset inventory

| Dataset | Holder | What it is | Access note | GAIA status |
| --- | --- | --- | --- | --- |
| Harmonized Landsat Sentinel-2 | NASA | Surface reflectance. Landsat 8/9 at 30 m, 16-day repeat. Sentinel-2 at 10–20 m, about 5-day repeat. | NASA Earthdata; also Microsoft Planetary Computer. Compute is not free. | not ingested |
| MERRA-2 | NASA | Reanalysis, about 40 years of global weather fields. | NASA-hosted. | not ingested |
| Landsat 8/9 in Copernicus Browser | USGS program, served by Copernicus | Level-1, 2021 to present, published about one month after acquisition. | View open. Remote download restricted to Copernicus services and collaborative ground segments. | not ingested |
| ERA5 | ECMWF / Copernicus Climate Change Service | Global reanalysis, hourly, commonly used as the weather baseline beside MERRA-2. | Copernicus Climate Data Store. Terms are the CDS license, not recorded here. | not ingested |
| Sentinel-2 | ESA | Optical land imaging. The MSI half of HLS. | Copernicus Data Space. | not ingested |
| GBIF | GBIF secretariat | Species occurrence records. Not an Earth-image feed. | Requires citation of the download DOI. | not ingested |

A row without a recorded license text is NeedVerify for reuse. This table records access notes, not a license grant.

## Planetary boundaries, literature only

Richardson et al., Science Advances, 13 Sep 2023, "Earth beyond six of nine planetary boundaries." The update finds six of the nine boundaries transgressed. Ocean acidification is close. Aerosol loading exceeds the boundary regionally. Stratospheric ozone has slightly recovered. Transgression is not an overnight state change. The nine processes are the 2009 Rockström framework as updated in 2015 and 2023: climate, biosphere integrity, land-system change, freshwater, biogeochemical flows, ocean acidification, atmospheric aerosol, stratospheric ozone, novel entities.

GAIA does not emit a boundary score. A corpus sentence that says the system measures planetary health is Tier C until a control variable, a threshold, and a source series are named.

## Twin rung

NASA ESTO, October 2024, defines an Earth System Digital Twin as three capabilities: a replica of past and current state, forecasts from that replica, and what-if scenarios. The same briefing lists uncertainty quantification as a required technology, and the roadmap at that date was prototypes, not an operational planetary twin.

GAIA's current rung is below that. The crate test is a fixture ensemble. It has no replica, no forecast from observations, and no scenario fed by a sensor. A maturity claim without a validation procedure stays Tier C.

## Refusal

No Copernicus, NOAA, NASA, ECMWF, GBIF, or DestinE client. No training run.
