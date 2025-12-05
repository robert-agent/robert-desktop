<script lang="ts">
  import ContextSidebar from './components/ContextSidebar.svelte';
  import ChatArea from './components/ChatArea.svelte';
  import './app.postcss';
</script>

<main class="flex h-screen w-screen bg-gray-950 text-white overflow-hidden">
  <ContextSidebar />
  <ChatArea />
</main>
