<svelte:head>
	<title>Home</title>
</svelte:head>

<script>
	let units = [
        "Unit #1",
        "Unit #2",
        "Unit #3",
        "Unit #4",
        "Unit #5",
        "Unit #6",
    ];
    let test = [
        "Graphing",
        "Functions",
        "Algebra",
        "Computing",
        "Data",
        "Polynomials",
    ];
</script>

<main>
    <!-- Have an each statement here that puts all the units into the list -->
	<ul>
        {#each units as unit, i}
            <!-- svelte-ignore a11y-invalid-attribute -->
            <li><a href="lesson" data-text={test[i]}>{unit}</a></li>
        {/each}
    </ul>
</main>

<style>
	@import url('https://fonts.googleapis.com/css?family=Poppins:200,300,400,500,600,700,800,900&display=swap'); 
    ul { 
        position: relative; 
    } 
    ul li { 
        list-style: none; 
        text-align: center;
    } 
    ul li a { 
        color: #333; 
        text-decoration: none; 
        font-size: 3em; 
        padding: 5px 20px; 
        display: inline-flex; 
        font-weight: 700; 
        transition: 0.5s; 
    } ul:hover li a { 
        color: #0002; 
    } ul li:hover a { 
        color: #000; 
        background: rgba(255,255,255,1);
        border-radius: 10px;
    } ul li a:before { 
        content: ''; 
        position: absolute; 
        top: 50%; 
        left: 40%; 
        transform: translate(-50%,-50%); 
        display: flex; 
        justify-content: center; 
        align-items: center; 
        font-size: 3em; 
        color: rgba(0,0,0,.1); 
        z-index: -1; 
        opacity: 0; 
        font-weight: 900; 
        text-transform: uppercase; 
        letter-spacing: 500px; 
        transition: letter-spacing 0.5s,left 0.5s; 
    } ul li a:hover:before { 
        content: attr(data-text); 
        opacity: 1; 
        left: 50%; 
        letter-spacing: 10px; 
        width: 1800px; 
        height: 1800px; 
    }  ul li:nth-child(6n+1) a:before { 
        background: #81ecec; 
    } ul li:nth-child(6n+2) a:before { 
        background: #ff7675; 
    } ul li:nth-child(6n+3) a:before { 
        background: #55efc4; 
    } ul li:nth-child(6n+4) a:before { 
        background: #a29bfe; 
    } ul li:nth-child(6n+5) a:before { 
        background: #fd79a8; 
    } ul li:nth-child(6n+6) a:before { 
        background: #ffeaa7;
    }
	main {
        font-family: 'Poppins', sans-serif;
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 1rem;
		margin: 0 auto;
		box-sizing: border-box;
	}
</style>
