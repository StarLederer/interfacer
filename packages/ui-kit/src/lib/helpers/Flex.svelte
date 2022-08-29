<script lang="ts">
  export let direction: "row" | "column" | null = null;
  export let justify: "space-between" | "center" | "flex-start" | "flex-end" | null = null;
  export let align: "stretch" | "center" | "flex-start" | "flex-end" | null = null;
  export let gap: number | null = 0.4;
</script>

<div style={`
  ${direction !== null ? `flex-direction: ${direction};` : ''}
  ${justify !== null ? `justify-content: ${justify};` : ''}
  ${align !== null ? `align-items: ${align};` : ''}
  ${gap !== null ? `gap: ${gap}rem;` : ''}
`}>
 <slot />
</div>

<style>
  div {
    display: flex;
  }
</style>
