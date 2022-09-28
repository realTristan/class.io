<svelte:head>
	<title>MHF4UI - Dashboard</title>
</svelte:head>

<script>
	const SIDEBAR_NAMES = [
		"Classes",
		"Students",
		"Settings",
		"Analytics"
	]
	const FAKE_NAMES = [
		"Tristan",
		"Ella",
		"Keeleigh",
		"James",
		"Connor",
		"Michael",
		"Andrew"
	]
	const FAKE_CLASSES = [
		"MHF4UI",
		"PHYSICS",
		"CHEMISTRY"
	]
	function rand_image() {
		let rand_num = Math.floor(Math.random() * (500 - 400) + 400)
		return `https://picsum.photos/${rand_num}/${rand_num}`
	}
	function get_fake_name() {
		return FAKE_NAMES[Math.floor(Math.random()*FAKE_NAMES.length)]
	}
</script>

<main>
	<h1>Dashboard</h1>

	<ul>
        {#each SIDEBAR_NAMES as name}
            <!-- svelte-ignore a11y-invalid-attribute -->
            <li style="display: flex; justify-content; center;">
                <a href="/dashboard"
                    data-text="VIEW_{name}">{name}
                </a>
            </li>
        {/each}
    </ul>
	
	<h1>My Classes</h1>
	<div style="margin-left: 30px;">
		{#each FAKE_CLASSES as class_name}
			<div style="display: inline-block; margin: 20px;">
				<h3 style="color: #333; font-weight: 900; border-radius: 8px;">{class_name}</h3>
				<div style="width: 300px; height: 200px; background-color: white; border-radius: 10px;"></div>
				<div> 
					<h3 class="class_bottom">2022/08/07</h3>
					<h3 class="class_bottom">Analytics</h3>
				</div>
			</div>
		{/each}
	</div>
	
	<h1>Students</h1>
	{#each FAKE_CLASSES as class_name}
		<div style="margin-left: 40px;">
			<h2 style="color: #333; opacity: 1; letter-spacing: 1px; text-transform: uppercase;">{class_name}</h2>
			<div style="margin-left: 30px;">
				{#each Array(20) as _}
					<div style="display: inline-block; margin: 20px;">
						<div style="text-align: center; position: relative;">
							<img
								style="width: 100px; height: 100px; background-color: white; border-radius: 100px;" 
								src={rand_image()}
								alt=""
							/>
							<div class="overlay">
								<div class="remove_text">Remove</div>
							</div>
							<h3 style="color: #333; opacity: 1; letter-spacing: 1px; text-transform: uppercase;">{get_fake_name()}</h3>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/each}
</main>

<style>
	@import url('https://fonts.googleapis.com/css?family=Poppins:700,800,900&display=swap');
    
	.class_bottom {
		display: inline-block;
		background-color: #00ff73;
		font-weight: 800;
		color: #333;
		padding-top: 5px;
		padding-bottom: 5px;
		padding-left: 15px;
		padding-right: 20px;
		border-radius: 8px;
		margin-right: 10px;
	}
	
	main {
        overflow: hidden;
        font-family: 'Poppins', sans-serif;
		flex: 1;
		padding: 1rem;
		margin: 0 auto;
	}
	h1 {
		font-size: 40px;
		font-family: 'Poppins', sans-serif;
        color: #333;
        font-weight: 900;
        text-transform: uppercase;
    }
	.overlay {
		cursor: pointer;
		position: absolute;
		top: 0;
		width: 100px;
		height: 100px; 
		background-color: white; 
		border-radius: 100px;
		opacity: 0;
		transition: .3s ease;
		background-color: #000000;
	}
	.overlay:hover { opacity: 0.8; }
	.remove_text {
		color: white;
		font-size: 15px;
		font-weight: 700;
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
	}

    /* On Hover Effect */
    ul { 
        position: relative; 
    } ul li {
        list-style: none;
        text-align: center;
    } ul li a {
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
        font-size: 2em;
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
    #staticSelectionHover:not(:hover) a:before {
        content: attr(data-text);
        right: 0px;
        opacity: 1;
        letter-spacing: 10px;
    } #staticSelectionHover:not(:hover) a {
        letter-spacing: 6px;
        color: #000;
        background: rgba(255,255,255,1);
        border-radius: 10px;
    }
	/* Scroll Bar */
	:root::-webkit-scrollbar {
		width: 20px;
	}
	:root::-webkit-scrollbar-track {
		background: #f1f1f1; 
	}
	:root::-webkit-scrollbar-thumb {
		background: #6366f1;
	}
	:root::-webkit-scrollbar-thumb:hover {
		background: #474af2;
	}
</style>
