<svelte:head>
	<title>Home</title>
</svelte:head>

<script>
    let unitNames = [
        "Graphing",
        "Functions",
        "Algebra",
        "Computing",
        "Data",
        "Polynomials",
        "Vectors",
        "Calculus",
        "Properties"
    ];
    let colors = [
        "#81ecec",
        "#ff7675",
        "#55efc4",
        "#a29bfe",
        "#fd79a8",
        "#ffeaa7",
        "#81ecec",
        "#ff7675",
        "#55efc4"
    ];
</script>

<main>
    <h1 style="color: #7c3aed; font-weight: 900; letter-spacing: 2px; margin-left: 20px; margin-bottom: -1px;">
        <mark style="color: black; background: none;">Mr.Simpson's</mark> MHF4UI
    </h1>
    <div>
        <ul>
            {#each unitNames as unit, i}
                <!-- svelte-ignore a11y-invalid-attribute -->
                <li style="display: flex; justify-content; center; letter-spacing: 1px;">
                    <a href="/lesson/{unit.toLowerCase()}" data-text={unit} style="color: #7c3aed;">
                        <mark style="color: black; background: none;">&nbsp; >>&nbsp; Unit #{i+1}&nbsp&nbsp </mark>{unit}
                    </a>
                </li>
            {/each}
        </ul>
    </div>
</main>

<style>
	@import url('https://fonts.googleapis.com/css?family=Poppins:200,300,400,500,600,700,800,900&display=swap'); 
    ul { position: relative; } 
    ul li {
        list-style: none;
        text-align: center;
    } 
    ul li a {
        color: #333;
        text-decoration: none;
        font-size: 2.5em;
        padding: 5px 20px;
        display: inline-flex;
        font-weight: 900;
        transition: 0.5s;
    } ul:hover li a { 
        color: #0002;
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
    } ul li:hover a {
        letter-spacing: 5px;
    }
	main {
        overflow: hidden;
        font-family: 'Poppins', sans-serif;
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 1rem;
		margin: 0 auto;
	}
</style>
