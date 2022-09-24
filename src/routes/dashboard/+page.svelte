<svelte:head>
	<title>MHF4UI - Dashboard</title>
</svelte:head>

<script>
	import { onMount } from 'svelte';
	import Dropdown from './components/dropdown.svelte'
	import JsonData from './settings_data.json'
	
	// The openCloseSettingSlider() function is used to
	// open / close the homework question sliders and 
	// change the contents within said sliders to hidden.
	function openCloseSettingSlider(dropDownTitles, dropDownDescriptions, e, i) {
		// If slider is closed, open it
		if (e.style.height == '20px' || e.style.height == '') {
			e.style.height = `${
				30*(dropDownDescriptions[i].innerHTML.length/50)
			}px`;
			e.style.display = "block";
			dropDownDescriptions[i].style.display = 'block';

			// Close all the other equation dropDownDescriptions
			for (let n = 0; n < dropDownTitles.length; n++) {
				if (n != i) {
					dropDownTitles[n].style.height = '20px';
					dropDownDescriptions[n].style.display = 'none';
				}
			}
		} 
		// Else, Hide the slider
		else {
			e.style.height = '20px';
			dropDownDescriptions[i].style.display = 'none';
		}
	}
	// Call the contents in this function when
	// the website is mounted (aka loaded)
	onMount(() => {
		let dropDownTitles = [];
		let dropDownDescriptions = [];

		// For each of the slide elements
		document.querySelectorAll("#fade_in_text").forEach((e) => {
			e.style.display = 'none';
			dropDownDescriptions.push(e);
		});

		// For each of the slide elements
		document.querySelectorAll("#slider").forEach((e, i) => {
			dropDownTitles.push(e);

			// Establish the event listener
			e.addEventListener("click", () => openCloseSettingSlider(
				dropDownTitles, dropDownDescriptions, e, i
			));
		});
	});
</script>

<main>
	<h1>MHF4UI Dashboard</h1>

	{#each Object.entries(JsonData) as [key, value]}
		<Dropdown title={key} description={value}/>
	{/each}
	<div class="seperator"></div>
</main>

<style>
	@import url('https://fonts.googleapis.com/css?family=Poppins:700,900&display=swap'); 

	/* Lesson Seperator Line */
	.seperator {
		margin-top: 120px; 
		margin-bottom: 120px;
	}
	/* Main Styles */
	main {
		text-align: center;
		padding: 1em;
		margin: 0 auto;
		font-family: 'Poppins', sans-serif;
	}
	h1 {
		font-family: 'Poppins', sans-serif;
        color: #333;
        font-weight: 900;
        text-transform: uppercase;
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
