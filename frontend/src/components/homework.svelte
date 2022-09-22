<script>
    export let LessonData;

	// Store the alphabet for equations
	const ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
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
						<div id="fade_in_text">
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
	#fade_in_text {
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