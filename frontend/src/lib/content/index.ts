import berandaJson from '../../../../content/pages/beranda.json';
import sejarahJson from '../../../../content/pages/sejarah.json';
import visiMisiJson from '../../../../content/pages/visi-misi.json';
import strukturJson from '../../../../content/pages/struktur-organisasi.json';
import petaJson from '../../../../content/pages/peta-wilayah.json';

import type {
  Beranda,
  Sejarah,
  VisiMisi,
  StrukturOrganisasi,
  PetaWilayah
} from './types';

export function getBeranda(): Beranda {
  return berandaJson as Beranda;
}

export function getSejarah(): Sejarah {
  return sejarahJson as Sejarah;
}

export function getVisiMisi(): VisiMisi {
  return visiMisiJson as VisiMisi;
}

export function getStrukturOrganisasi(): StrukturOrganisasi {
  return strukturJson as StrukturOrganisasi;
}

export function getPetaWilayah(): PetaWilayah {
  return petaJson as PetaWilayah;
}

export type {
  Beranda,
  Sejarah,
  VisiMisi,
  StrukturOrganisasi,
  PetaWilayah
} from './types';
