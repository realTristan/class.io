<svelte:head>
	<title>MHF4UI - Selection</title>
</svelte:head>

<script>
    import { onMount } from 'svelte';

    // Unit Names
    const UNIT_NAMES = [
        "Graphing", "Functions", "Algebra",
        "Computing", "Data", "Polynomials",
        "Vectors", "Calculus", "Properties"
    ];
    // Unit Colors
    const COLORS = [
        "#81ecec", "#ff7675", "#55efc4",
        "#a29bfe", "#fd79a8", "#ffeaa7",
        "#81ecec", "#ff7675", "#55efc4"
    ];

    // Configuring the starting unit selection styling
    var onLoadRandOptNum, onLoadSelection;
    onMount(() => {
        onLoadRandOptNum = Math.floor(Math.random() * UNIT_NAMES.length);
        onLoadSelection = true;
    })
</script>

<main>
    <h1 style="color: white; font-weight: 900; letter-spacing: 2px; margin-left: 20px; margin-bottom: -1px;">
        <mark style="color: #333; background: none; ">Mr.Simpson's</mark> MHF4UI
    </h1>
    <ul 
        on:mouseenter={() => { if (onLoadSelection) onLoadSelection = false; }}
        on:mouseleave={() => { if (!onLoadSelection) {
            onLoadRandOptNum = Math.floor(Math.random() * UNIT_NAMES.length);
            onLoadSelection = true;
        }}}
    >
        {#each UNIT_NAMES as unit, i}
            <!-- svelte-ignore a11y-invalid-attribute -->
            <li
                id={ i == onLoadRandOptNum && onLoadSelection ? "onLoadHover":"" } 
                style="display: flex; justify-content; center;"
                onmouseover="body.style.background='{COLORS[i]}';">
                <a href="lesson"
                    data-text={unit}>Unit #{i+1}&nbsp;:&nbsp; {unit}
                </a>
            </li>
        {/each}
    </ul>
</main>

<style>
	@import url('https://fonts.googleapis.com/css?family=Poppins:200,300,400,500,600,700,800,900&display=swap'); 
    main {
        overflow: hidden;
        font-family: 'Poppins', sans-serif;
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 1rem;
		margin: 0 auto;
	}
    /* On Hover Effect */
    ul { position: relative; } 
    ul li {
        list-style: none;
        text-align: center;
    } 
    ul li a {
        color: #0002;
        text-decoration: none;
        font-size: 2.5em;
        padding: 5px 20px;
        display: inline-flex;
        font-weight: 900;
        transition: 0.5s;
        letter-spacing: 2px;
    } ul:hover li a { 
        color: #0002;
    } ul li:hover a {
        color: #000;
        background: rgba(255,255,255,1);
        border-radius: 10px;
        letter-spacing: 6px;
    } ul li a:before {
        content: '';
        position: absolute;
        top: 30%;
        margin-right: 50px;
        display: flex;
        justify-content: center;
        align-items: center;
        font-size: 3em;
        color: rgba(0, 0, 0, .1);
        z-index: -1;
        font-weight: 900;
        text-transform: uppercase;
        letter-spacing: 500px;
        transition: letter-spacing 0.5s, left 0.5s;
    } ul li:hover a:before {
        content: attr(data-text);
        right: 0px;
        opacity: 1;
        letter-spacing: 10px;
    }
    /* On Page Load Temporary Unit Hover */
    #onLoadHover:not(:hover) a:before {
        content: attr(data-text);
        right: 0px;
        opacity: 1;
        letter-spacing: 10px;
    } #onLoadHover:not(:hover) a {
        letter-spacing: 6px;
        color: #000;
        background: rgba(255,255,255,1);
        border-radius: 10px;
    }
</style>
