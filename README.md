# Heart Rate App
The goal of this app is to evaluate Z2 performance on runs. This can be done by
looking at the average HR for the first half of the run, as compared to the average
HR over the second half.

Garmin web app does not allow this feature (only average HR over entire run).

## Outline
tcx file -> csv
use tracks.csv file? (hopefully this contains the graph data)
Take (HeartRateBpm, Time) columns
As a simple implementation, grab the first 50% of rows, which should be the first half of run,
then select the HeartRateBpm values and calculate a simple average
