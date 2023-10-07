import { defineConfig } from 'unocss';
import { presetUno } from '@unocss/preset-uno';
import presetIcons from '@unocss/preset-icons';

export default defineConfig({
    content: {
        filesystem: ['**/*.{html,js,ts,jsx,tsx,rs}'],
    },
    presets: [
        presetUno(),
        presetIcons({
            collections: {
                ph: () => import('@iconify-json/ph').then(i => i.icons),
            },
        }),
    ],
});
