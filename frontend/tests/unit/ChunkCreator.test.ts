/**
 * Unit tests for ChunkCreator component - TDD approach
 */
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent, waitFor } from '@testing-library/svelte';
import ChunkCreator from '$lib/components/ChunkCreator.svelte';

describe('ChunkCreator Component', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should render input field and button', () => {
    const { getByPlaceholderText, getByText } = render(ChunkCreator);
    
    expect(getByPlaceholderText('Enter text content')).toBeInTheDocument();
    expect(getByText('Create Chunk')).toBeInTheDocument();
  });

  it('should update input value when typing', async () => {
    const { getByPlaceholderText } = render(ChunkCreator);
    const input = getByPlaceholderText('Enter text content') as HTMLInputElement;
    
    await fireEvent.input(input, { target: { value: 'test content' } });
    
    expect(input.value).toBe('test content');
  });

  it('should show loading state when creating chunk', async () => {
    const { getByText, getByPlaceholderText } = render(ChunkCreator);
    const input = getByPlaceholderText('Enter text content');
    const button = getByText('Create Chunk');
    
    await fireEvent.input(input, { target: { value: 'test' } });
    await fireEvent.click(button);
    
    await waitFor(() => {
      expect(getByText('Creating...')).toBeInTheDocument();
    });
  });

  it('should disable button when input is empty', () => {
    const { getByText } = render(ChunkCreator);
    const button = getByText('Create Chunk') as HTMLButtonElement;
    
    expect(button.disabled).toBe(true);
  });

  it('should enable button when input has content', async () => {
    const { getByText, getByPlaceholderText } = render(ChunkCreator);
    const input = getByPlaceholderText('Enter text content');
    const button = getByText('Create Chunk') as HTMLButtonElement;
    
    await fireEvent.input(input, { target: { value: 'test' } });
    
    expect(button.disabled).toBe(false);
  });

  it('should display storage type selector', () => {
    const { getByLabelText } = render(ChunkCreator);
    
    expect(getByLabelText('Storage Type:')).toBeInTheDocument();
  });

  it('should allow selecting storage type', async () => {
    const { getByLabelText } = render(ChunkCreator);
    const select = getByLabelText('Storage Type:') as HTMLSelectElement;
    
    await fireEvent.change(select, { target: { value: 'disk' } });
    
    expect(select.value).toBe('disk');
  });
});
