<template>
  <div class="site">
    <header class="header">
      <h1>Site View</h1>
      <router-link to="/" class="nav-link">Home</router-link>
    </header>

    <main class="main-content">
      <p>Site View - Coming Soon</p>
      <p>Program: {{ $route.params.program }}</p>
    </main>
  </div>
</template>

<style scoped>
.site {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 2rem;
  border-bottom: 1px solid var(--border-color);
}

.main-content {
  flex: 1;
  padding: 2rem;
}
</style>

