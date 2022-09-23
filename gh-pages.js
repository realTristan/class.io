import { publish } from 'gh-pages';

publish(
    './build', 
    {
        branch: 'gh-pages',
        repo: 'https://github.com/realTristan/MHF4UI.git',
        user: {
            name: 'realTristan',
            email: 'heytristaann@gmail.com'
        },
        dotfiles: true
    },
    () => {
        console.log('Github Page: https://realtristan.github.io/MHF4UI/');
    }
);