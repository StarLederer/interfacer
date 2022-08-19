<div class="loader" />

<style>
  .loader {
    border: 0.2rem solid;
    border-top: 0.2rem solid blue;
    border-radius: 50%;
    width: 1rem;
    height: 1rem;
    animation: spin 2s linear infinite;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>
