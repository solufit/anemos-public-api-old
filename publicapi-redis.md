# public-api-redis scheme memo

## key earthquake-eventid_hour


EarthQuake eventid_list

## key earthquake-eventid_day


EarthQuake eventid_list

## key earthquake-detail-{event-id}

expire 24 hours

EarthQuake Details

## key earthquake-expire-hour-{eventid}

expire one hour

value eventid

## key earthquake-expire-day-{eventid}

expire 24 hour

value eventid