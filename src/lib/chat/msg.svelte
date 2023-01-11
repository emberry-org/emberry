<script lang="ts">
  import { onMount } from "svelte";
  import { getEmbed, parseContent } from "./msg.parser";

  // Standard message variables.
	export let sender: string;
	export let content: string;
  export let time: string;
  export let chain: boolean;

  let embed: { title: string, desc: string, icon?: string, url: string, preview?: string } | undefined = undefined;
  let embedHeight: number = 0;

  onMount(async () => {
    embed = await getEmbed(content);
  });

</script>


<!--  Message information  -->

<li class="msg">
	{#if !chain}
    <div class="profile-picture" />
    <h3>{ sender } <span>{ time }</span></h3>
  {/if}
  <div class="content">
    <p>{ @html parseContent(content) }</p>
  </div>
</li>

{#if embed} <!--  Embedded information  -->
  
<li class="embed">
	<div class="content" bind:clientHeight={embedHeight}>

    <a href={ embed.url } target="_blank" rel="noreferrer">{ embed.url }</a>
    <h2>
      {#if embed.icon && embed.icon.length > 0}
        <img class="favicon" src={ embed.icon } alt={ embed.title } on:error={ (e) => e.currentTarget.style.display = 'none' }>
      {/if}
      { embed.title }
    </h2>
    
    {#if embed.desc.length > 0}
      <p>{ embed.desc }</p>
    {/if}
    
  </div>

  {#if embed.preview && embed.preview.length > 0}
    <img class="preview"
      src={ embed.preview } 
      alt={ embed.title } 
      height={ embed.title.length == 0 && embed.desc.length == 0 ? 240 : embedHeight } 
      on:error={ (e) => e.currentTarget.style.display = 'none' }
    >
  {/if}
</li>

{/if}


<style lang="scss">
  // Normal message
  .msg {
    position: relative;
    width: 100%;
    height: fit-content;

    //margin-bottom: 6px;
    padding-left: 42px;
    font-weight: 400;

    .profile-picture {
      position: absolute;

      left: 0;
      top: 3px;
      width: 34px;
      height: 34px;

      background-color: var(--bg-100);
      background-size: contain;
      border-radius: 8px;

      user-select: none;
      -webkit-user-select: none;

      // Picture Translucent Outline
      &::after {
        content: "";

        width: 30px;
        height: 30px;

        position: absolute;
        top: 0;
        left: 0;

        border: 2px solid #ffffff10;
        border-radius: 8px;
      }
    }

    h3 {
      height: 24px;
      color: var(--text-base);
      font-weight: 500;
      font-size: 15px;
      margin: 8px 0 0 0;

      span {
        font-size: 11px;
        color: var(--text-secondary);
        font-weight: 500;
        margin-left: 10px;
      }
    }

    .content {
      //background-color: var(--bg-300);
      height: fit-content;
      width: fit-content;
      padding: 0;//padding: 0px 12px 0px 12px;
      margin: 0;//margin: 4px 0 0 0;
      border-radius: 10px;
      display: flex;
      flex-direction: column;

      p {
        height: fit-content;
        color: var(--text-base);
        font-size: 15px;
        white-space: pre-wrap;
        line-height: 20px;
        margin: 0;//margin: 10px 0 12px 0;

        :global(a) {
          font-weight: inherit;
          color: var(--primary);

          &:hover {
            text-decoration: underline;
          }
        }
      }
    }
  }

  // Embedded info underneath the message
  .embed {
    position: relative;
    width: fit-content;
    height: min-content;

    margin: 8px 0 6px 0;
    padding-left: 42px;
    display: flex;

    .content {
      background-color: var(--bg-100);
      height: fit-content;
      min-width: 240px;
      max-width: 480px;
      width: fit-content;
      padding: 0px 12px 0px 12px;
      margin-right: 16px;
      border-radius: 10px;
      display: flex;
      flex-direction: column;

      a {
        margin: 8px 0 6px 0;
        font-weight: inherit;
        font-size: 15px;
        overflow: hidden;
        text-overflow: ellipsis;
        color: var(--text-secondary-light);

        &:hover {
          text-decoration: underline;
        }
      }

      h2 {
        color: var(--text-base);
        font-weight: 400;
        font-size: 18px;
        line-height: 22px;
        margin-top: 0;
        display: flex;
        align-items: center;

        .favicon {
          width: 24px;
          height: 24px;

          margin-right: 8px;
          background-color: #ddd;
          border-radius: 6px;
          padding: 4px;

          user-select: none;
          -webkit-user-select: none;
        }
      }

      p {
        height: fit-content;
        color: var(--text-secondary);
        font-size: 15px;
        white-space: pre-wrap;
        line-height: 18px;
        margin: 0 0 15px 0;
      }
    }

    .preview {
      border-radius: 12px;
    }
  }
</style>
