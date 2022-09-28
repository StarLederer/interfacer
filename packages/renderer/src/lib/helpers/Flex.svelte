<script lang="ts">
  import type * as CSS from "csstype";

  let className: string = "";
  export { className as class };

  export let direction: CSS.Property.FlexDirection | null = null;
  export let justify: CSS.Property.JustifyContent | null = null;
  export let align: CSS.Property.AlignContent | null = null;
  export let gap: number = 0.4;
</script>

<div
  class={className}
  style={`
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
