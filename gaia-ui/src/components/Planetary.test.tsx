import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import Planetary, { queryScale } from './Planetary';

describe('Planetary', () => {
  it('lists scales and queries nothing', () => {
    render(<Planetary />);
    expect(screen.getByRole('heading', { name: /planetary/i })).toBeTruthy();
    expect(screen.getByText(/city live=false/)).toBeTruthy();
    expect(screen.getByText(/global live=false/)).toBeTruthy();
    expect(queryScale('global')).toBe('no registry');
  });
});
