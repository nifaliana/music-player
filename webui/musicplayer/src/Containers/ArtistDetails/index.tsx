import { useEffect } from "react";
import { useNavigate, useParams } from "react-router-dom";
import ArtistDetails from "../../Components/ArtistDetails";
import { useGetArtistQuery } from "../../Hooks/GraphQL";
import { useTimeFormat } from "../../Hooks/useFormat";
import { usePlayback } from "../../Hooks/usePlayback";

const ArtistDetailsPage = () => {
  const params = useParams();
  const { data, loading, refetch } = useGetArtistQuery({
    variables: {
      id: params.id!,
    },
    fetchPolicy: "network-only",
  });

  useEffect(() => {
    params.id && refetch();
  }, [params.id, refetch]);

  const { formatTime } = useTimeFormat();
  const navigate = useNavigate();
  const {
    play,
    pause,
    next,
    previous,
    nowPlaying,
    nextTracks,
    previousTracks,
    playArtistTracks,
    playNext,
    playTrackAt,
    removeTrackAt,
  } = usePlayback();
  const artist = !loading && data ? data.artist : {};
  const tracks =
    !loading && data
      ? data.artist.songs.map((track, index) => ({
          id: track.id,
          title: track.title,
          artist: track.artists.map((artist) => artist.name).join(", "),
          album: track.album.title,
          time: formatTime(track.duration! * 1000),
          cover: track.album.cover ? `/covers/${track.album.cover}` : undefined,
          artistId: track.artists[0].id,
          albumId: track.album.id,
          index,
        }))
      : [];
  const albums =
    !loading && data
      ? data.artist.albums.map((album) => ({
          id: album.id,
          title: album.title,
          artist: album.artist,
          cover: album.cover ? `/covers/${album.cover}` : undefined,
        }))
      : [];
  return (
    <ArtistDetails
      onBack={() => navigate(-1)}
      onClickLibraryItem={(item) => navigate(`/${item}`)}
      onPlay={() => play()}
      onPause={() => pause()}
      onNext={() => next()}
      onPrevious={() => previous()}
      onShuffle={() => {}}
      onRepeat={() => {}}
      artist={artist}
      tracks={tracks}
      albums={albums}
      nowPlaying={nowPlaying}
      nextTracks={nextTracks}
      previousTracks={previousTracks}
      onPlayArtistTracks={(artistId, shuffle, position) =>
        playArtistTracks({ variables: { artistId, position, shuffle } })
      }
      onPlayNext={(trackId) => playNext({ variables: { trackId } })}
      onPlayTrackAt={(position) => playTrackAt({ variables: { position } })}
      onRemoveTrackAt={(position) => removeTrackAt({ variables: { position } })}
    />
  );
};

export default ArtistDetailsPage;
