
/*******************************************************************************
*                                                                              *
*            The key for deciphering the 'Karsten-Lars Manuscript'             *
*                                     aka                                      *
*                     'Protracker Period Table Generator'                      *
*                                                                              *
*                                   -------                                    *
*                                                                              *
*                          Raylight/Powerline - 2013                           *
*                                                                              *
*                                  Rev.  1.0b                                  *
*                                                                              *
*                                   -------                                    *
*                                                                              *
*                   Number crunchers who made this possible                    *
*                                                                              *
*            eightbitbubsy            pipe               raylight              *
*                                                                              *
********************************************************************************

 Background: The period tables used in Protracker etc are non-trivial; i.e.
 attempting to generate them the 'correct' way, will fail to get the correct
 value on all of the 576 entries, no matter how one adjusts the parameters.

 Using the relation described in the Amiga Hardware Reference Manual -
 Period = Clock Frequency / (Desired Frequency x Sample Size) - one can
 calculate the period for any desired frequency. However, the period needs to
 be rounded or truncated to an integer, so the frequency will be approximate
 to the ideal one.

 Mysteriously, many entries in the table used in Protracker are not the closest
 approximate values possible. Seemingly, there's no clear pattern to this,
 further adding to this mystery. :D Besides the issue of needing to include the
 hard-coded table in binaries, it is unacceptable that people lie awake all
 night wondering what great secrets it may hold!

 So after a descent into near-certain insanity, and having analyzed this in...
 well you really don't wanna know about all the crazy methods, tables,
 visualizations... :D  ...I did find a decent way of generating the beloved
 table! And perhaps some insight into how and why the table came to be as it is.
                                                                          
 Let's have a look at the table to shed some light over why it defies sanity:

  .=====+=============================+=====+=============================.
  | -8  | -7  -6  -5  -4  -3  -2  -1  |  0  | +1  +2  +3  +4  +5  +6  +7  |
  '=====+=============================+=====+============================='
  |     |                             .-----|-----------------------------.
  | 907 | 900 894 887 881 875 868 862 | 856 | 850 844 838 832 826 820 814 |  0
--.-----|-----------------------------'     |                             |  
C>| 856 | 850 844 838 832 826 820 814 | 808 | 802 796 791 785 779 774 768 |  1
  | 808 | 802 796 791 785 779 774 768 | 762 | 757 752 746 741 736 730 725 |  2
  | 762 | 757 752 746 741 736 730 725 | 720 | 715 709 704 699 694 689 684 |  3
  | 720 | 715 709 704 699 694 689 684 | 678 |<674>670 665 660 655 651 646 |  4
  | 678 |<675>670 665 660 655 651 646 | 640 |<637>632 628 623 619 614 610 |  5
  | 640 |<636>632 628 623 619 614 610 | 604 | 601 597 592 588 584 580 575 |  6
  | 604 | 601 597 592 588 584 580 575 | 570 | 567 563 559 555 551 547 543 |  7
  | 570 | 567 563 559 555 551 547 543 | 538 | 535 532 528 524 520 516 513 |  8
  |     |                             .-----|-----------------------------|
  | 538 | 535 532 528 524 520 516 513 | 508 | 505 502 498<495>491 487 484 |  9
  |-----|-----------------------------'     |                             |
A>| 508 | 505 502 498<494>491 487 484 | 480 | 477 474 470 467 463 460 457 | 10
  | 480 | 477 474 470 467 463 460 457 | 453 | 450 447 444 441 437 434 431 | 11
B | 453 | 450 447 444 441 437 434 431 | 428 | 425 422 419 416 413 410 407 | 12
C>| 428 | 425 422 419 416 413 410 407 | 404 | 401 398 395 392 390 387 384 | 13
  | 404 | 401 398 395 392 390 387 384 | 381 | 379 376 373 370 368 365 363 | 14
  | 381 | 379 376 373 370 368 365 363 | 360 | 357 355 352 350 347 345 342 | 15
  | 360 | 357 355 352 350 347 345 342 | 339 | 337 335 332 330 328 325 323 | 16
  | 339 | 337 335 332 330 328 325 323 | 320 | 318 316 314 312 309 307 305 | 17
  | 320 | 318 316 314 312 309 307 305 | 302 | 300 298 296 294 292 290 288 | 18
  | 302 | 300 298 296 294 292 290 288 | 285 | 284 282 280 278 276 274 272 | 19
  | 285 | 284 282 280 278 276 274 272 | 269 | 268 266 264 262 260 258 256 | 20
  | 269 | 268 266 264 262 260 258 256 | 254 | 253 251 249 247 245 244 242 | 21
A>| 254 | 253 251 249 247 245 244 242 | 240 |<239>237 235 233 232 230 228 | 22
  | 240 |<238>237 235 233 232 230 228 | 226 | 225<224>222 220 219 217 216 | 23
B | 226 | 225<223>222 220 219 217 216 | 214 |<213>211 209 208 206 205<204>| 24
C>| 214 |<212>211 209 208 206 205<203>| 202 |<201>199 198 196 195 193 192 | 25
  | 202 |<200>199 198 196 195 193 192 | 190 | 189 188 187 185 184 183 181 | 26
  | 190 | 189 188 187 185 184 183 181 | 180 | 179 177 176 175 174 172 171 | 27
  | 180 | 179 177 176 175 174 172 171 | 170 | 169 167 166 165 164 163 161 | 28
  | 170 | 169 167 166 165 164 163 161 | 160 | 159 158 157 156 155 154 152 | 29
  | 160 | 159 158 157 156 155 154 152 | 151 | 150 149 148 147 146 145 144 | 30
  | 151 | 150 149 148 147 146 145 144 | 143 | 142 141 140 139 138 137 136 | 31
  | 143 | 142 141 140 139 138 137 136 | 135 | 134 133 132 131 130 129 128 | 32
  |     |                             .-----|-----------------------------|
  | 135 | 134 133 132 131 130 129 128 | 127 | 126 125 125<124>123 122 121 | 33
  |-----|-----------------------------'     |                             |
A>| 127 | 126 125 125<123>123 122 121 | 120 | 119 118 118 117 116 115 114 | 34
  | 120 | 119 118 118 117 116 115 114 | 113 | 113 112 111 110 109 109 108 | 35
  '-----'-----------------------------'-----'-----------------------------'
 
 I've organized this in a way that shows the important aspects. We have
 36 rows - 3 octaves of low C to high B. Apart from the normal note 
 frequencies, the middle column, we also have detuned versions - 16 in total.
 As shown, 8 fine tune steps equals 1 semitone - the leftmost column has
 the same values as the normal tuning (middle column).

 Now, if you'd try to generate the values for normal tuning, they would
 match the table from 113 (high B) to 508 (low A), but the higher values
 would be off in both directions. The fine versions would display a 
 different more random pattern of error.

 It is my belief this is because of the history behind this table. In the 
 beginning, i.e. in Ultimate Soundtracker by Karsten Obarski, there was 
 no fine tuning - only the 36 values seen in the middle column. At that point 
 in time, the Amiga Hardware Reference Manual was still in its first edition. 
 In this edition, the periods for 2 octaves of A to B - for NTSC - are listed. 
 (There is no direct mention of NTSC or PAL there either). Those values are the
 ones from 135 to 508, which also happen to be the ones that would match if
 you calculated them. If we look at the extended range - 113 to 127 and
 538 to 856 - it becomes apparent that those are exactly double resp. half
 of the corresponding values in the AHRM, _instead_ of being the closest
 approximation. It is likely that this was the way they came to be. Besides the
 obvious - why build a space ships just for adding 12 more number in a piece of
 source code - the approach of doubling has its benefits sound-wise regarding
 relative pitch.

 ------------------------------------------------------------------------------

 In any case, the range became 3 octaves of C to B. So what about the fine
 tuned versions? Trying the same philosophy on them does not work. In fact,
 despite showing a more typical number distribution of something generated,
 trying to calculate them typically ends in some numbers that match, others
 don't.

 To understand why, we again turn to... erhm.. speculate about.. ;) history!
 Fine-tuning (and Protracker) saw the light at a later time, by authors with
 their own way on how to approach things, and 576 values is a pretty good 
 incentive to write some code that generates them for you! And so they were. 
 What's important here though is that the ones (normal tune) inherited from 
 the past could not be altered as it would have broken existing modules: 
 The full table is actually a superimposition. The 576 values are generated 
 by one algorithm, based on one author's preference, but the old existing 
 values are used on all entries in which they apply. (The period 907 makes 
 this particularly evident; it would be 906 if simply doubled, as previously 
 done.)

 But even with this, we still can't generate the correct values! The author
 chose to divide the fine-tuning as 1/8 of a semitone. Our wisdom tells us
 that f(tune) = f(0)*2^(1/12 * tune/8). So we pick the frequency for normal 
 tuning and apply the formula. For example: C = 523.3 Hz, which we calculate 
 to be period 428 as in ARHM. Let's try fine-tune +2 on this one:

 C                                            = 523.3 Hz
 C(+2)          = 523.3 * 2^(1/12*2/8)        = 530.9 Hz
 C(+2) period   = 3579545 / (530.9... * 16)   = 421.39     -> period 421

 The answer should have been 422 to match the table. No matter how we round
 or tweak we can't get all entries to match using the same logic as for the
 normal tuning.

 It turns out that, for some reason, Protracker did NOT use the exact 
 frequency of a reference note when creating the table. Instead, the actual 
 RESULTING frequency of a reference note's period was (likely) used as the 
 reference! So:

 C period       = 428
 "C"            = 3579545/428/16              = 522.7 Hz
 "C"(+2)        = 523.3 * 2^(1/12*2/8)        = 530.3 Hz 
 "C"(+2) period = 3579545 / (530.9... * 16)   = 421.88     -> period 422

 The result is that the fine-tuned periods are "skewed" with a factor of about 
 1001/1000 from ones based on the ideal frequencies of notes. Based on my 
 insanity, it looks like "C", calculated from period 856, is the reference 
 used. Or alternatively, the actual frequency of B or C as written in the AHRM. 
 (522.700 Hz, 553.800 Hz)

 ------------------------------------------------------------------------------

 So to calculate the table you need to:

 1.  Calculate all 576 entries using period 856 (or resulting NTSC frequency)
     as reference.
 2a. Calculate normal tuning using an actual reference note (e.g A = 440.0 Hz)
     to get periods 113 to 508.
 2b. Periods 538 to 856 are double their respective periods one octave above.
 3.  Replace the periods from step one with the ones from step two for tuning 0
     and tuning -8 (where possible, i.e. not period 907)
 4.  Oh yes, there is one little thing left! :D

 ------------------------------------------------------------------------------

 Ok, can we please go home now? No, not yet! Doing the above will generate an
 accurate table, _except_ for 9 entries. If you look at the table above you
 will see them shown as <###>. As would be expected, tuning -8 to -1 and 0
 to 7 are copies of each other, just shifted by one semitone. However, there
 are 9 pairs that refuse to be a part of this. And this is the final unsolved
 mystery! ;) Some options:

 1. They are typos.
 2. The table has had some manual adjustment.
 3. They are a result of limited precision, error propagation when doing 
    fixed-point.
 4. My guess of how the "Protracker part" of the table was generated is
    incorrect or incomplete.
 5. A combination of the above.
 6. They exist to force you to sacrifice 18 bytes or so to correct them.
 7. There lies a deeper mystery hidden in the manuscript. ;)

 ------------------------------------------------------------------------------
 
 Revision 1.0b - things to do:

 - Figure out the reason for the mystery entries!
 - Implement a fixed point version.
 - Estimate the size of an 68k assembler implementation.
 - Implement a tiny 68k assembler version.
 - Add some interesting information from observations made regarding the
   periods. :D

*******************************************************************************
*                                                                             *
*                 You can go home now - only code below!                      *
*                                                                             *
*******************************************************************************/

#include <stdio.h>
#include <stdlib.h>

/*******************************************************************************
 * Nothing to see here, move along...                                          *
 ******************************************************************************/

enum PeriodType
{
	TYPE_UST_AHRM    =  1,
	TYPE_UST_DOUBLED =  2,
	TYPE_UST_HALVED  =  4,
	TYPE_PT          =  8,
	TYPE_PT_MYSTERY  = 16
};

double pow2(double x)
{ 
	return pow(2.0, x); 
}

/*******************************************************************************
 *                                                                             *
 * getPTPeriod - returns the correct period value given a note and a fine-tune *
 *               value. Somewhat pedagogical reference implementation.         *
 *               note   = 0 .. 35                                              *
 *               tune = -8 .. +7                                               *
 *                                                                             *
 * Algorithm: 1. A normalized period value is calculated (period(0,0) == 1.0). *
 *            2. The vale is multiplied by the PT reference period, unless the *
 *               entry existed in UST. If so, the UST/AHRM reference period is *
 *               used instead.                                                 *
 *            3. If the entry is a UST one and also one the lower periods not  *
 *               found in AHRM 1st ed (i.e. 536 .. 856), the period is taken   *
 *               from respective period one octave higher, multiplied by two.  *
 *            4. Finally - for PT entries -an annoying sequence of manual      *
 *               corrections are applied for the stubborn entries described    *
 *               above!                                                        *
 *                                                                             *
 ******************************************************************************/

int getPTPeriod(int note, int tune)
{
	const double NTSC_CLK       = 3579545.0;
	const double REF_PERIOD_PT  = 856.0;
	const double REF_PERIOD_UST = NTSC_CLK / 523.3 / 8;

	// Convert note and tune into are more helpful representation

	int note2                   = note + (tune + 8) / 8;
	int tune2                   = (tune + 8) & 7;

	// (1) Calculate the normalized period, i.e. period <= 1.0

	double period               = pow2((double)-tune / 8.0 * 1.0/12.0);
	       period              *= pow2((double)-note / 12.0);

	// (2) Select between the PT and UST for the wanted entry

	if(tune2 == 0 && note2 != 0)
	{
		period *= REF_PERIOD_UST;

		// (3) Perform the equivalent of taking resulting entry 
		//     "period/2", and multiply by 2 for periods above 508

		if(note2 < 10)
		{
			period = (double)((int)((period + 1.0) / 2.0) * 2);
		}
	}
	else
	{
		period *= REF_PERIOD_PT;

		// (4) Super efficient manual correction of the evil nine

		period = (tune == -7 && note ==  6) ? period - 1 : period;
		period = (tune == -7 && note == 26) ? period - 1 : period;
		period = (tune == -4 && note == 34) ? period - 1 : period;
		period = (tune ==  1 && note ==  4) ? period - 1 : period;
		period = (tune ==  1 && note == 22) ? period + 1 : period;
		period = (tune ==  1 && note == 24) ? period + 1 : period;
		period = (tune ==  2 && note == 23) ? period + 1 : period;
		period = (tune ==  4 && note ==  9) ? period + 1 : period;
		period = (tune ==  7 && note == 24) ? period + 1 : period;
	}

	return (int)(period + 0.5);
}

/*******************************************************************************
 *                                                                             *
 * getPTPeriodType - Returns the "type" of a particular entry - for debugging, *
 *                   verification, visualization, or whatnot.                  *
 *                                                                             *
 ******************************************************************************/

enum PeriodType getPTPeriodType(int note, int tune)
{
	int note2 = note + (tune + 8) / 8;
	int tune2 = (tune + 8) & 7;

	if(tune2 == 0 && note2 != 0)
	{
		if(note2 < 10)	return TYPE_UST_DOUBLED;
		if(note2 > 33)	return TYPE_UST_HALVED;

		return TYPE_UST_AHRM;
	}

	// (Note: There are 9 pairs of values)

	if(tune2 == 1 && note2 ==  6) return TYPE_PT_MYSTERY;
	if(tune2 == 1 && note2 == 26) return TYPE_PT_MYSTERY;
	if(tune2 == 4 && note2 == 34) return TYPE_PT_MYSTERY;
	if(tune2 == 1 && note2 ==  5) return TYPE_PT_MYSTERY;
	if(tune2 == 1 && note2 == 23) return TYPE_PT_MYSTERY;
	if(tune2 == 1 && note2 == 25) return TYPE_PT_MYSTERY;
	if(tune2 == 2 && note2 == 24) return TYPE_PT_MYSTERY;
	if(tune2 == 4 && note2 == 10) return TYPE_PT_MYSTERY;
	if(tune2 == 7 && note2 == 25) return TYPE_PT_MYSTERY;

	return TYPE_PT;
}

/*******************************************************************************
 * generatePTPeriodTable                                                       *
 *                                                                             *
 * Generates all 576 period values. Somewhat more efficient reference          *
 * implementation. Generating all entries at once is more close to how a       *
 * optimized (and/or fixed point) implementation would work. It does NOT       *
 * generate the entries "strictly PT sequentially", i.e. you can't output      *
 * entries one by one in the same order as defined in Protracker as they are   *
 * being generated, since the implementation is geared towards filling a       *
 * 572 word table efficiently. This algorithm is written in a way to           *
 * represent the reasoning in the introduction above, and not to be the        *
 * best approach. :D                                                           *
 *                                                                             *
 * Table organization:                                                         *
 *                                                                             *
 * Columns - periods in descending order as in Protracker etc.                 *
 * Rows    - fine tune in the same order as in Protracker etc,                 *
 *           i.e. 0 .. 7, -8 .. -1                                             *
 *                                                                             *
 ******************************************************************************/

void generatePTPeriodTable(unsigned short (*periods)[36])
{
	const double NTSC_CLK        = 3579545.0;
	const double REF_PERIOD_PT   = 856.0;
	const double REF_PERIOD_UST  = NTSC_CLK / 523.3 / 8;
	const double UST_TO_PT_RATIO = REF_PERIOD_UST / REF_PERIOD_PT;
	const double semitone_step   = pow2(-1.0/12.0);
	const double tune_step       = pow2(-1.0/8.0 * 1.0/12.0); 
	int n, t;

	// Initialize with starting period, i.e. 907

	double p1 = REF_PERIOD_PT / semitone_step;

	for(t = 0 ; t < 8 ; t++)
	{
		// Initialize with starting period for current tuning

		double p2 = p1;

		for(n = 0 ; n < 36 ; n++)
		{
			// Round and save current period, update period for next Semitone

			periods[t+8][n]   = (unsigned short)(p2 + 0.5);
			p2               *= semitone_step;
			periods[t][n]     = (unsigned short)(p2 + 0.5);

			// Save correct UST period for normal tuning

			if(t == 0)
			{
				periods[0][n] = (unsigned short)(p2 * UST_TO_PT_RATIO + 0.5);
			}
		}

		// Starting period for next tuning

		p1 *= tune_step;
	}

	// Create correct values for the octave halved periods for normal tuning

	for(n = 0 ; n < 9 ; n++)   { periods[0][n] = periods[0][n+12] * 2; }

	// Copy UST periods to tuning -8

	for(n = 1 ; n < 36 ; n++)  { periods[8][n] = periods[0][n-1];      }

	// Correct those 9 #?!$?#!%!! entries that refuse

	periods[1][ 4]--;  periods[1][22]++;  periods[ 1][24]++;
	periods[2][23]++;  periods[4][ 9]++;  periods[ 7][24]++;
	periods[9][ 6]--;  periods[9][26]--;  periods[12][34]--;
}

/*******************************************************************************
 *                                                                             *
 * generatePTPeriodTable2 (WIP)                                                *
 *                                                                             *
 * Generates all 576 period values. Slightly optimized/obfuscated version.     *
 * Note that this one requires a buffer size of 576+1. Highly commented code   *
 * for your convenience!                                                       *
 *                                                                             *
 ******************************************************************************/

void generatePTPeriodTable2(unsigned short* periods)
{
	const double NTSC_CLK        = 3579545.0;
	const double REF_PERIOD_PT   = 856.0;
	const double REF_PERIOD_UST  = 3579545.0 / 523.3 / 8;
	const double UST_TO_PT_RATIO = REF_PERIOD_UST / REF_PERIOD_PT;
	const double semitone_step   = pow2(-1.0/12.0);
	const double tune_step       = pow2(-1.0/8.0 * 1.0/12.0); 
	int n, t;
	
	unsigned short* periods2     = periods + 36*8;
	int idx                      = 0;
	double p1                    = REF_PERIOD_PT / semitone_step;

	for(t = 0 ; t < 8 ; t++)
	{
		double p2      = p1;
		periods2[idx]  = (unsigned short)(p2 + 0.5);

		for(n = 1 ; n < 36+1 ; n++)
		{
			           p2              *= semitone_step;
			           periods2[++idx]  = (unsigned short)(p2 + 0.5);
			if(t == 0) periods2[idx  ]  = (unsigned short)(p2 * UST_TO_PT_RATIO + 0.5);
			           periods [idx-1]  = periods2[idx];
		}

		p1 *= tune_step;
	}

	for(n = 0 ; n < 9 ; n++)
	{ 
		periods2[n+1] = periods[n] = periods[n+12] * 2; 
	}

	periods[ 40]--;  periods[ 58]++;  periods[ 60]++;
	periods[ 95]++;  periods[153]++;  periods[276]++;
	periods[330]--;  periods[350]--;  periods[466]--;
}

/*******************************************************************************
 *                                                                             *
 * ...And the rest is just stuff to output something :)                        *
 *                                                                             *
 ******************************************************************************/

void printPTPeriodTableHeader()
{
	int t;
	for(t = 0 ; t < 8 ; t++)  { printf("  %d  ", t);    }
	for(t = 0 ; t < 8 ; t++)  { printf(" %d  ", t - 8); }
	printf("\n");
}

void printPTPeriodTable(unsigned short (*periodTable)[36])
{
	int n, t;

	for(n = 0 ; n < 36 ; n++)
	{
		for(t = 0 ; t < 16 ; t++)
		{
			printf("%d  ", periodTable[t][n]);
		}
		printf("\n");
	}
}

void printPTPeriodTable2(unsigned short* periods)
{
	int n, t;

	for(n = 0 ; n < 36 ; n++)
	{
		for(t = 0 ; t < 16 ; t++)
		{
			printf("%d  ", periods[t*36 + n]);
		}
		printf("\n");
	}
}

/*******************************************************************************
 *                                                                             *
 * I have NO idea what this "main" function is, but it seems to be needed.     *
 *                                                                             *
 ******************************************************************************/

int main(int argc, char* argv[])
{
	int n, t;
	unsigned short periodTable[16][36];
	unsigned short periodTable2[16*36 + 1];

	printf("Table Entry Classification\n");
	printf("--------------------------\n");
	printPTPeriodTableHeader();

	for(n = 0 ; n < 36 ; n++)
	{
		for(t = -8 ; t < 8 ; t++)
		{
			int tp = getPTPeriodType(n, t);

			switch(tp)
			{
				case TYPE_UST_AHRM:    printf("AHRM "); break;
				case TYPE_UST_DOUBLED: printf(" x2  "); break;
				case TYPE_UST_HALVED:  printf(" /2  "); break;
				case TYPE_PT:          printf(" PT  "); break;
				case TYPE_PT_MYSTERY:  printf("!PT! "); break;
				default:                                break;
			}
		}
		printf("\n");
	}

	printf("\nTable generated by getPTPeriod");
	printf("\n------------------------------\n");
	printPTPeriodTableHeader();

	for(n = 0 ; n < 36 ; n++)
	{
		for(t = -8 ; t < 8 ; t++)
		{
			printf("%d", getPTPeriod(n, t));
			if(getPTPeriodType(n, t) == TYPE_PT_MYSTERY)  printf("! ");
			else                                          printf("  ");
		}

		printf("\n");
	}

	printf("\nTable generated by generatePTPeriodTable");
	printf("\n----------------------------------------\n");
	printPTPeriodTableHeader();

	generatePTPeriodTable(&periodTable[0]);
	printPTPeriodTable(&periodTable[0]);

	printf("\nTable generated by generatePTPeriodTable2");
	printf("\n-----------------------------------------\n");
	printPTPeriodTableHeader();

	generatePTPeriodTable2(&periodTable2[0]);
	printPTPeriodTable2(&periodTable2[0]);

	system("pause");
	return 0;
}
/*
 1.0   - Initial Release
 1.0b  - Added credits to those it belongs. Corrected some typos.
_______________________________________________________________________________
)______________________________________________________________________________)
|856850844838832826820814907900894887881875868862808802796791785779774768856850|
|844838832826820814762757752746741736730725808802796791785779774768720715709704|
|699694689684762757752746741736730725678674670665660655651646720715709704699694|
|689684640637632628623619614610678675670665660655651646604601597592588584580575|
|64063663262862361961461057056756355955555154754360460159759258858458057553853/
|55325285245205165135705675635595555515475435085055024984954914874845385355325|
|28524520516513480477474470467463460457508505502498494491487484453450447444441|
|43743443148047747447046746346045742842542241941641341040745345044744444143743 )
|4431404401398395392390387384428425422419416413410407381379376373370368365363 /
|4044013983953923903873843603573553523503473453423813793763733703683653633393|
|3733533233032832532336035735535235034734534232031831631431230930730533933733|
|5332330328325323302300298296294292290288320318316314312309307305285284282280|
|2782762742723023002982962942922902882692682662642622602582562852842822802782|
|7627427225425325124924724524424226926826626426226025825624023923723523323223 )
|0228254253251249247245244242226225224222220219217216240238237235233232230228(  
|21421321120920820620520422622522322222021921721620220119919819619519319221421\
|22112092082062052031901891881871851841831812022001991981961951931921801791771 )
|76175174172171190189188187185184183181170169167166165164163161180179177176175 |
|17417217116015915815715615515415217016916716616516416316115115014914814714614 |
|51441601591581571561551541521431421411401391381371361511501491481471461451441 )
|35134133132131130129128143142141140139138137136127126125125124123122121135134(
|133132131130129128120119118118117116115114127126125125123123122121113113112111\
|11010910 9108 1201 19 118 11 8   11 7 1 1    6 1   1 5     1      1          4 )
`------- --- ---- - ---- ---- -------- -- ---------------  --- -- -------------´
*/