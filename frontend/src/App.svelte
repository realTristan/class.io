<script>
	import { onMount } from 'svelte';
	import LessonData from '../course_data.json'
	import Homework from './components/homework.svelte'
	
	// The openCloseEquationSlider() function is used to
	// open / close the homework question sliders and 
	// change the contents within said sliders to hidden.
	function openCloseEquationSlider(equations, solutions, e, i) {
		// If slider is closed, open it
		if (e.style.height == '20px' || e.style.height == '') {
			e.style.height = '60px';
			e.style.display = "block";
			solutions[i].style.display = 'block';

			// Close all the other equation solutions
			for (let n = 0; n < equations.length; n++) {
				if (n != i) {
					equations[n].style.height = '20px';
					solutions[n].style.display = 'none';
				}
			}
		} 
		// Else, Hide the slider
		else {
			e.style.height = '20px';
			solutions[i].style.display = 'none';
		}
	}
	// Call the contents in this function when
	// the website is mounted (aka loaded)
	onMount(() => {
		let equations = [];
		let solutions = [];

		// For each of the slide elements
		document.querySelectorAll("#fade_in_text").forEach((e) => {
			e.style.display = 'none';
			solutions.push(e);
		});

		// For each of the slide elements
		document.querySelectorAll("#slider").forEach((e, i) => {
			equations.push(e);

			// Establish the event listener
			e.addEventListener("click", () => openCloseEquationSlider(
				equations, solutions, e, i
			));
		});
	});

</script>

<main>
	{#each Object.entries(LessonData) as [key]}
		{@const LESSON_TITLE_SPLIT = LessonData[key]["title"].split(" |")}

		<!-- Example: Lesson #8 Factor Theorum -->
		<h1>
			<a style="color: #7c3aed;"
				rel="noopener noreferrer" target="_blank"
				href={LessonData[key]["note"]}> 
				<mark style="color: black; background: none;">
					<!-- Example: Lesson #8 -->
					{LESSON_TITLE_SPLIT[0]}
				</mark>
					<!-- Example: Factor Theorum -->
					{LESSON_TITLE_SPLIT[1]}
			</a>
		</h1>
		
		<!-- Lesson Info -->
		<div>
			<iframe
				style="border-radius: 5px;"
				width="480" height="220" frameborder="0"
				src={LessonData[key]["video"]["url"]} 
				title={LessonData[key]["video"]["title"]} 
				allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen>
			</iframe>
		</div>

		<!-- Homework Title -->
		<h2>
			<a style="color: #7c3aed;"
				rel="noopener noreferrer" target="_blank"
				href={LessonData[key]["hw_full_solutions"]}>
				<mark style="color: black; background: none;">{LESSON_TITLE_SPLIT[0]}</mark> Homework
			</a>
		</h2>

		<!-- Homework Questions -->
		<Homework LessonData={LessonData[key]}/>
		<div class="seperator"></div>
	{/each}
</main>

<style>
	/* Lesson Seperator Line */
	.seperator {
		height: 2px; 
		background-color: #d4d4d8; 
		margin-top: 120px; 
		margin-bottom: 120px; 
		margin-left: 100px; 
		margin-right: 100px;
	}
	/* Main Styles */
	main {
		text-align: center;
		padding: 1em;
		margin: 0 auto;
	}
	h1 {
        text-align: center;
        color: #8b5cf6;
        font-size: 4em;
        font-weight: 700;
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