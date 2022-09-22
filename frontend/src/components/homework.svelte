<script>
    import { onMount } from 'svelte';
    export let LessonData;

	// Store the alphabet for equations
	const ALPHABET = "abcdefghijklmnopqrsrtuvwxyz";

	// Store drop down display values
	let dropDownDisplays = [];
	
	// The openCloseEquationSlider() function is used to
	// open / close the homework question sliders and 
	// change the contents within said sliders to hidden.
	function openCloseEquationSlider(se, e, i) {
		// If slider is closed, open it
		if (e.style.height == '20px' || e.style.height == '') {
			e.style.height = '60px';
			dropDownDisplays[i] = "block";

			// Close all the other equation solutions
			for (let n = 0; n < se.length; n++) {
				if (n != i) {
					se[n].style.height = '20px'; 
					dropDownDisplays[n] = "none"
				}
			}
		} 
		// Else, Hide the slider
		else {
			e.style.height = '20px'; 
			dropDownDisplays[i] = "none"
		}
	}
	// Call the contents in this function when
	// the website is mounted (aka loaded)
	onMount(() => {
		// SlideElements
		let se = [];

		// For each of the slide elements
		document.querySelectorAll("#slider").forEach((e, i) => {
			se.push(e);
			// Add the drop down display value to the array
			dropDownDisplays[i] = "none";

			// Establish the event listener
			e.addEventListener("click", () => openCloseEquationSlider(se, e, i));
		});
	});
</script>

<main>
	<!-- Homework Questions -->
	{#each LessonData["hw_questions"] as section, n}
		<h3>{section.title}</h3>
		<div style="margin-left: 20%; margin-right: 20%;">
			<!-- Create new dropdown div for each question -->
			{#each section.questions as equation, i}
				<div style="padding: 10px; display: inline-block; cursor: pointer; font-weight: 600;">
					<div id="slider">

						<!-- Set the equation -->
						<mark style="color: #3f3f46; background: none; font-weight: 600;">{ALPHABET[i]}) </mark>
							{equation}
						
						<!-- Set the equation solution -->
						<div class="fade-in-text" style="display: {dropDownDisplays[i]};">
							<div style="margin: 10px;">
								<mark style="color: black; background: none; font-weight: 600;">&nbsp;&nbsp;Correct Answer: </mark>
								<mark style="color: #3f3f46; background: none; font-weight: 600;">&nbsp;{LessonData["hw_solutions"][n]["questions"][i]}</mark>
							</div>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/each}
</main>

<style>
	/* Homework Dropdown Slider Content FadeIn */
	.fade-in-text {
		animation: fadeIn 5s;
	}
	@keyframes fadeIn {
		0% { opacity: 0; }
		20% { opacity: 1; }
	}
	
	/* Homework Dropdown Slider */
	#slider {
		min-width: 400px;
		margin: 0 auto; 
		text-align: left;
		color: #7c3aed;
		border-radius: 7px;
		margin-top: 20px;
		background-color: white;
		box-shadow: 0.5px 0.5px 10px 0.5px rgba(0, 0, 0, 0.1);
		padding: 12px 16px;
		z-index: 1;
		height: 20px;
		transition:             height 500ms ease;
			-moz-transition:    height 500ms ease;
			-ms-transition:     height 500ms ease;
			-o-transition:      height 500ms ease;
			-webkit-transition: height 500ms ease;
	}
</style>