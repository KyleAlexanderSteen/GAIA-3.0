const SCALES = [
  { id: 'city', live: false, person: false },
  { id: 'country', live: false, person: false },
  { id: 'continent', live: false, person: false },
  { id: 'global', live: false, person: false },
] as const;

export function queryScale(_id: string): string {
  return 'no registry';
}

export default function Planetary() {
  return (
    <section aria-label="Planetary interface">
      <h2>Planetary</h2>
      <p>Scales are names. Nothing is queried.</p>
      <ul>
        {SCALES.map((scale) => (
          <li key={scale.id}>
            {scale.id} live={String(scale.live)} person={String(scale.person)} query={queryScale(scale.id)}
          </li>
        ))}
      </ul>
    </section>
  );
}
